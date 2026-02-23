//! Prolog language executor — port of `languages/prolog.py`.
//!
//! Turbo-Prolog-flavoured subset: facts, rules with comma-body,
//! queries with backtracking, cut, basic arithmetic predicates.

use crate::context::{ControlFlow, ExecContext, PrologFact, PrologRule};
use std::collections::HashMap;

// ── entry point ────────────────────────────────────────────────────────────────

pub fn execute_prolog(ctx: &mut ExecContext, line: &str) -> ControlFlow {
    let line = line.trim();

    if line.is_empty() || line.starts_with('%') || line.starts_with("/*") {
        return ControlFlow::Continue;
    }

    // Accumulate multi-line clauses
    ctx.prolog_buffer.push_str(line);
    ctx.prolog_buffer.push(' ');

    // Only process when we have a complete clause (ends with `.`)
    if !ctx.prolog_buffer.trim_end().ends_with('.') {
        return ControlFlow::Continue;
    }

    let clause = ctx.prolog_buffer.trim().to_string();
    ctx.prolog_buffer.clear();

    // Query: ?- functor(args).
    if clause.starts_with("?-") {
        let inner = &clause[2..clause.len()-1].trim();
        return exec_query(ctx, inner);
    }

    // Rule: head :- body.
    if clause.contains(":-") {
        return assert_rule(ctx, &clause);
    }

    // Fact: functor(args).
    return assert_fact(ctx, &clause);
}

// ── fact assertion ─────────────────────────────────────────────────────────────

fn assert_fact(ctx: &mut ExecContext, clause: &str) -> ControlFlow {
    let clause = clause.trim().trim_end_matches('.');
    if let Some((functor, args)) = parse_term(clause) {
        ctx.prolog_facts.push(PrologFact { functor, args: parse_terms(&args) });
    }
    ControlFlow::Continue
}

// ── rule assertion ─────────────────────────────────────────────────────────────

fn assert_rule(ctx: &mut ExecContext, clause: &str) -> ControlFlow {
    let clause = clause.trim().trim_end_matches('.');
    if let Some(pos) = clause.find(":-") {
        let head = &clause[..pos].trim();
        let body = &clause[pos+2..].trim();
        if let Some((functor, params_str)) = parse_term(head) {
            let params = parse_terms(&params_str);
            let body_goals: Vec<(String, Vec<String>)> = split_body_goals(body)
                .into_iter()
                .filter(|g| !g.is_empty() && g != "!")
                .filter_map(|g| {
                    if let Some((f, args)) = parse_term(&g) {
                        Some((f, parse_terms(&args)))
                    } else if !g.is_empty() {
                        Some((g.clone(), vec![]))
                    } else {
                        None
                    }
                })
                .collect();
            ctx.prolog_rules.push(PrologRule { functor, params, body: body_goals });
        }
    }
    ControlFlow::Continue
}

// ── query execution ────────────────────────────────────────────────────────────

fn exec_query(ctx: &mut ExecContext, query: &str) -> ControlFlow {
    let query = query.trim();

    // Parse: functor(args)
    let (functor, args_str) = match parse_term(query) {
        Some(t) => t,
        None => {
            ctx.emit(&format!("❌ Prolog syntax: {query}\n"));
            return ControlFlow::Continue;
        }
    };
    let args = parse_terms(&args_str);

    // Collect all solutions
    let facts = ctx.prolog_facts.clone();
    let rules = ctx.prolog_rules.clone();

    let solutions = resolve_goal(&functor, &args, &facts, &rules, HashMap::new(), 0);

    // Built-in predicates
    if solutions.is_empty() && functor == "write" && args.len() == 1 {
        let val = resolve_var(&args[0], &HashMap::new());
        ctx.emit(&format!("{val}\n"));
        return ControlFlow::Continue;
    }

    // Output solutions
    if solutions.is_empty() {
        ctx.emit("false.\n");
    } else {
        // Extract variable bindings
        let vars: Vec<String> = args.iter().filter(|a| is_var(a) && *a != "_").cloned().collect();
        if vars.is_empty() {
            ctx.emit("true.\n");
        } else {
            for sol in &solutions {
                let mut parts = Vec::new();
                for v in &vars {
                    if let Some(val) = sol.get(v.as_str()) {
                        parts.push(format!("{v} = {val}"));
                    }
                }
                if !parts.is_empty() {
                    ctx.emit(&format!("{}\n", parts.join(", ")));
                }
            }
            ctx.emit(&format!("({} solution{})\n", solutions.len(), if solutions.len() == 1 { "" } else { "s" }));
        }
    }

    ControlFlow::Continue
}

// ── recursive goal resolution ─────────────────────────────────────────────────

const MAX_DEPTH: usize = 32;

/// Resolve a single goal against facts and rules, returning all solution environments.
fn resolve_goal(
    functor: &str,
    args: &[String],
    facts: &[PrologFact],
    rules: &[PrologRule],
    env: HashMap<String, String>,
    depth: usize,
) -> Vec<HashMap<String, String>> {
    if depth > MAX_DEPTH { return vec![]; }

    let mut solutions = Vec::new();

    // Resolve args through current environment
    let resolved: Vec<String> = args.iter().map(|a| resolve_var(a, &env)).collect();

    // Match against facts
    for fact in facts {
        if fact.functor == functor && fact.args.len() == resolved.len() {
            if let Some(new_env) = unify_terms(&resolved, &fact.args, env.clone()) {
                solutions.push(new_env);
            }
        }
    }

    // Match against rules (recursive)
    for rule in rules {
        if rule.functor == functor && rule.params.len() == resolved.len() {
            let renamed = rename_vars(rule);
            if let Some(rule_env) = unify_terms(&resolved, &renamed.params, env.clone()) {
                // Attempt to solve all body goals
                let mut envs = vec![rule_env];
                for (goal_f, goal_args) in &renamed.body {
                    let mut next_envs = Vec::new();
                    for e in envs {
                        let goal_resolved: Vec<String> = goal_args.iter()
                            .map(|a| resolve_var(a, &e))
                            .collect();
                        let sub_solutions = resolve_goal(
                            goal_f, &goal_resolved, facts, rules, e, depth + 1,
                        );
                        next_envs.extend(sub_solutions);
                    }
                    envs = next_envs;
                    if envs.is_empty() { break; }
                }
                solutions.extend(envs);
            }
        }
    }

    solutions
}

// ── unification ────────────────────────────────────────────────────────────────

fn unify_terms(
    query_args: &[String],
    fact_args:  &[String],
    mut env:    HashMap<String, String>,
) -> Option<HashMap<String, String>> {
    if query_args.len() != fact_args.len() { return None; }
    for (q, f) in query_args.iter().zip(fact_args.iter()) {
        env = unify(q, f, env)?;
    }
    Some(env)
}

fn unify(x: &str, y: &str, mut env: HashMap<String, String>) -> Option<HashMap<String, String>> {
    let x = resolve_var(x, &env);
    let y = resolve_var(y, &env);

    if x == "_" || y == "_" { return Some(env); }
    if x == y   { return Some(env); }

    if is_var(&x) {
        env.insert(x, y);
        return Some(env);
    }
    if is_var(&y) {
        env.insert(y, x);
        return Some(env);
    }
    None
}

fn resolve_var(term: &str, env: &HashMap<String, String>) -> String {
    resolve_var_depth(term, env, 0)
}

fn resolve_var_depth(term: &str, env: &HashMap<String, String>, depth: u32) -> String {
    if depth > 64 { return term.to_string(); }
    if is_var(term) {
        if let Some(v) = env.get(term) {
            return resolve_var_depth(v, env, depth + 1);
        }
    }
    term.to_string()
}

fn is_var(term: &str) -> bool {
    if term.is_empty() { return false; }
    if term == "_" { return true; }
    term.chars().next().map_or(false, |c| c.is_uppercase() || c == '_')
}

// ── rename variables in a rule (for fresh instances) ─────────────────────────

static RENAME_CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn rename_vars(rule: &PrologRule) -> PrologRule {
    let suffix = RENAME_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let rename = |s: &String| -> String {
        if is_var(s) { format!("{s}_{suffix}") } else { s.clone() }
    };
    PrologRule {
        functor: rule.functor.clone(),
        params: rule.params.iter().map(|p| rename(p)).collect(),
        body: rule.body.iter().map(|(f, args)| {
            (f.clone(), args.iter().map(|a| rename(a)).collect())
        }).collect(),
    }
}

// ── term parser ────────────────────────────────────────────────────────────────

/// Split a rule body by commas that are NOT inside parentheses.
fn split_body_goals(body: &str) -> Vec<String> {
    let mut goals = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    for ch in body.chars() {
        if ch == '(' { depth += 1; }
        if ch == ')' { depth -= 1; }
        if ch == ',' && depth == 0 {
            let g = current.trim().to_string();
            if !g.is_empty() { goals.push(g); }
            current.clear();
        } else {
            current.push(ch);
        }
    }
    let g = current.trim().to_string();
    if !g.is_empty() { goals.push(g); }
    goals
}

/// Parse `functor(args_str)` or `functor` (arity 0).
fn parse_term(s: &str) -> Option<(String, String)> {
    let s = s.trim();
    if let Some(op) = s.find('(') {
        let functor = s[..op].trim().to_string();
        let rest = &s[op+1..];
        let cp = rest.rfind(')')?;
        let args = rest[..cp].trim().to_string();
        if !functor.is_empty() { return Some((functor, args)); }
    } else if s.chars().next().map_or(false, |c| c.is_alphanumeric() || c == '_') {
        return Some((s.to_string(), String::new()));
    }
    None
}

fn parse_terms(args: &str) -> Vec<String> {
    if args.trim().is_empty() { return vec![]; }
    let mut result = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    for ch in args.chars() {
        if ch == '(' { depth += 1; }
        if ch == ')' { depth -= 1; }
        if ch == ',' && depth == 0 {
            let t = current.trim().to_string();
            if !t.is_empty() { result.push(t); }
            current.clear();
        } else {
            current.push(ch);
        }
    }
    let t = current.trim().to_string();
    if !t.is_empty() { result.push(t); }
    result
}
