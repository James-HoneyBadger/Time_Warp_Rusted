//! Comprehensive integration tests for ALL Time Warp language interpreters.
//! Tests every command, operator, function, and feature in every language.

use tw_core::interpreter::Interpreter;
use tw_core::language::Language;
use tw_core::interpreter::RunState;

/// Helper: Run a program in a given language and return the output string.
/// Runs up to max_steps batches, auto-provides "42" for any input requests.
fn run_program(lang: Language, source: &str) -> String {
    let mut interp = Interpreter::new(lang);
    interp.load(source);
    interp.run();
    for _ in 0..500 {
        let still_running = interp.step_batch();
        if matches!(interp.state, RunState::WaitingInput) {
            interp.provide_input("42");
            continue;
        }
        if !still_running {
            break;
        }
    }
    interp.output()
}

/// Helper: Run with specific inputs provided in order.
fn run_with_inputs(lang: Language, source: &str, inputs: &[&str]) -> String {
    let mut interp = Interpreter::new(lang);
    interp.load(source);
    interp.run();
    let mut input_idx = 0;
    for _ in 0..500 {
        let still_running = interp.step_batch();
        if matches!(interp.state, RunState::WaitingInput) {
            let val = inputs.get(input_idx).copied().unwrap_or("0");
            interp.provide_input(val);
            input_idx += 1;
            continue;
        }
        if !still_running {
            break;
        }
    }
    interp.output()
}

/// Helper: Check the interpreter finishes (doesn't hang).
fn run_finishes(lang: Language, source: &str) -> bool {
    let mut interp = Interpreter::new(lang);
    interp.load(source);
    interp.run();
    for _ in 0..500 {
        let still_running = interp.step_batch();
        if matches!(interp.state, RunState::WaitingInput) {
            interp.provide_input("0");
            continue;
        }
        if !still_running {
            return true;
        }
    }
    false // still running after 500 batches = hung
}

/// Helper: Get turtle state after running a program.
fn run_get_turtle(lang: Language, source: &str) -> (f64, f64, f64, bool, usize, usize) {
    let mut interp = Interpreter::new(lang);
    interp.load(source);
    interp.run();
    for _ in 0..500 {
        let still_running = interp.step_batch();
        if !still_running { break; }
    }
    let t = &interp.ctx.turtle;
    (t.x, t.y, t.heading, t.pen_down, t.lines.len(), t.shapes.len())
}

// ═══════════════════════════════════════════════════════════════════════════════
// BASIC INTERPRETER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn basic_print_string() {
    let out = run_program(Language::Basic, r#"PRINT "Hello World""#);
    assert!(out.contains("Hello World"), "PRINT string failed: {out}");
}

#[test]
fn basic_print_number() {
    let out = run_program(Language::Basic, "PRINT 42");
    assert!(out.contains("42"), "PRINT number failed: {out}");
}

#[test]
fn basic_print_semicolon_no_newline() {
    let out = run_program(Language::Basic, "PRINT \"A\";\nPRINT \"B\"");
    assert!(out.contains("AB"), "Semicolon should suppress newline: {out}");
}

#[test]
fn basic_print_expression() {
    let out = run_program(Language::Basic, "PRINT 3 + 4 * 2");
    assert!(out.contains("11"), "Expression eval failed: {out}");
}

#[test]
fn basic_let_assignment() {
    let out = run_program(Language::Basic, "LET X = 10\nPRINT X");
    assert!(out.contains("10"), "LET assignment failed: {out}");
}

#[test]
fn basic_implicit_assignment() {
    let out = run_program(Language::Basic, "X = 25\nPRINT X");
    assert!(out.contains("25"), "Implicit assignment failed: {out}");
}

#[test]
fn basic_input() {
    let out = run_with_inputs(Language::Basic, "INPUT \"Enter: \"; X\nPRINT X", &["99"]);
    assert!(out.contains("99"), "INPUT failed: {out}");
}

#[test]
fn basic_if_then_true() {
    let out = run_program(Language::Basic, "X = 5\nIF X > 3 THEN PRINT \"YES\"");
    assert!(out.contains("YES"), "IF/THEN true branch failed: {out}");
}

#[test]
fn basic_if_then_false() {
    let out = run_program(Language::Basic, "X = 1\nIF X > 3 THEN PRINT \"YES\"");
    assert!(!out.contains("YES"), "IF should not execute false branch: {out}");
}

#[test]
fn basic_if_then_else() {
    let out = run_program(Language::Basic, "X = 1\nIF X > 3 THEN PRINT \"BIG\" ELSE PRINT \"SMALL\"");
    assert!(out.contains("SMALL"), "IF/ELSE failed: {out}");
}

#[test]
fn basic_block_if_else_endif() {
    let out = run_program(Language::Basic, "X = 10\nIF X > 5 THEN\nPRINT \"A\"\nELSE\nPRINT \"B\"\nEND IF\nPRINT \"C\"");
    assert!(out.contains("A"), "Block IF should take true branch: {out}");
    assert!(out.contains("C"), "Should continue after END IF: {out}");
    assert!(!out.contains("B"), "Should not execute ELSE branch: {out}");
}

#[test]
fn basic_block_if_false_else() {
    let out = run_program(Language::Basic, "X = 1\nIF X > 5 THEN\nPRINT \"A\"\nELSE\nPRINT \"B\"\nEND IF\nPRINT \"C\"");
    assert!(!out.contains("A"), "Should skip IF body: {out}");
    assert!(out.contains("B"), "Should take ELSE branch: {out}");
    assert!(out.contains("C"), "Should continue after END IF: {out}");
}

#[test]
fn basic_for_next() {
    let out = run_program(Language::Basic, "FOR I = 1 TO 5\nPRINT I;\nNEXT I");
    assert!(out.contains("1") && out.contains("5"), "FOR/NEXT failed: {out}");
}

#[test]
fn basic_for_step() {
    let out = run_program(Language::Basic, "FOR I = 0 TO 10 STEP 2\nPRINT I;\nNEXT");
    assert!(out.contains("0") && out.contains("10"), "FOR/STEP failed: {out}");
}

#[test]
fn basic_while_wend() {
    let out = run_program(Language::Basic, "X = 0\nWHILE X < 5\nX = X + 1\nWEND\nPRINT X");
    assert!(out.contains("5"), "WHILE/WEND failed: {out}");
}

#[test]
fn basic_do_loop_while() {
    let out = run_program(Language::Basic, "X = 0\nDO\nX = X + 1\nLOOP WHILE X < 3\nPRINT X");
    assert!(out.contains("3"), "DO/LOOP WHILE failed: {out}");
}

#[test]
fn basic_do_loop_until() {
    let out = run_program(Language::Basic, "X = 0\nDO\nX = X + 1\nLOOP UNTIL X >= 3\nPRINT X");
    assert!(out.contains("3"), "DO/LOOP UNTIL failed: {out}");
}

#[test]
fn basic_goto() {
    let out = run_program(Language::Basic, "GOTO SKIP\nPRINT \"BAD\"\nSKIP:\nPRINT \"GOOD\"");
    assert!(out.contains("GOOD"), "GOTO failed: {out}");
    assert!(!out.contains("BAD"), "GOTO didn't skip: {out}");
}

#[test]
fn basic_gosub_return() {
    let out = run_program(Language::Basic, "GOSUB MYSUB\nPRINT \"BACK\"\nEND\nMYSUB:\nPRINT \"IN SUB\"\nRETURN");
    assert!(out.contains("IN SUB"), "GOSUB didn't call sub: {out}");
    assert!(out.contains("BACK"), "RETURN didn't come back: {out}");
}

#[test]
fn basic_sub_definition() {
    let out = run_program(Language::Basic, "CALL GREET\nEND\nSUB GREET\nPRINT \"HI\"\nEND SUB");
    assert!(out.contains("HI"), "SUB definition/CALL failed: {out}");
}

#[test]
fn basic_function_definition() {
    let out = run_program(Language::Basic, "FUNCTION DOUBLE(X)\nDOUBLE = X * 2\nEND FUNCTION\nPRINT DOUBLE(7)");
    assert!(out.contains("14"), "FUNCTION definition failed: {out}");
}

#[test]
fn basic_select_case() {
    let out = run_program(Language::Basic, "X = 2\nSELECT CASE X\nCASE 1\nPRINT \"ONE\"\nCASE 2\nPRINT \"TWO\"\nCASE ELSE\nPRINT \"OTHER\"\nEND SELECT");
    assert!(out.contains("TWO"), "SELECT CASE failed: {out}");
    assert!(!out.contains("ONE"), "SELECT CASE should not match 1: {out}");
}

#[test]
fn basic_dim_array() {
    let out = run_program(Language::Basic, "DIM A(5)\nA(3) = 99\nPRINT A(3)");
    assert!(out.contains("99"), "DIM/array failed: {out}");
}

#[test]
fn basic_cls() {
    assert!(run_finishes(Language::Basic, "CLS\nPRINT \"OK\""));
}

#[test]
fn basic_color_command() {
    assert!(run_finishes(Language::Basic, "COLOR 4\nPRINT \"RED\""));
}

#[test]
fn basic_width_command() {
    assert!(run_finishes(Language::Basic, "WIDTH 3\nPRINT \"OK\""));
}

#[test]
fn basic_rem_comment() {
    let out = run_program(Language::Basic, "REM This is a comment\nPRINT \"OK\"");
    assert!(out.contains("OK"), "REM comment failed: {out}");
}

#[test]
fn basic_apostrophe_comment() {
    let out = run_program(Language::Basic, "' This is a comment\nPRINT \"OK\"");
    assert!(out.contains("OK"), "' comment failed: {out}");
}

#[test]
fn basic_colon_separator() {
    let out = run_program(Language::Basic, "X = 5 : PRINT X");
    assert!(out.contains("5"), "Colon separator failed: {out}");
}

#[test]
fn basic_string_variable() {
    let out = run_program(Language::Basic, "A$ = \"HELLO\"\nPRINT A$");
    assert!(out.contains("HELLO"), "String variable failed: {out}");
}

#[test]
fn basic_str_function() {
    let out = run_program(Language::Basic, "PRINT STR$(42)");
    assert!(out.contains("42"), "STR$ failed: {out}");
}

#[test]
fn basic_chr_function() {
    let out = run_program(Language::Basic, "PRINT CHR$(65)");
    assert!(out.contains("A"), "CHR$ failed: {out}");
}

#[test]
fn basic_left_function() {
    let out = run_program(Language::Basic, "PRINT LEFT$(\"HELLO\", 3)");
    assert!(out.contains("HEL"), "LEFT$ failed: {out}");
}

#[test]
fn basic_right_function() {
    let out = run_program(Language::Basic, "PRINT RIGHT$(\"HELLO\", 3)");
    assert!(out.contains("LLO"), "RIGHT$ failed: {out}");
}

#[test]
fn basic_mid_function() {
    let out = run_program(Language::Basic, "PRINT MID$(\"HELLO\", 2, 3)");
    assert!(out.contains("ELL"), "MID$ failed: {out}");
}

#[test]
fn basic_string_concat() {
    let out = run_program(Language::Basic, "PRINT \"HEL\" + \"LO\"");
    assert!(out.contains("HELLO"), "String concat failed: {out}");
}

#[test]
fn basic_end_statement() {
    let out = run_program(Language::Basic, "PRINT \"A\"\nEND\nPRINT \"B\"");
    assert!(out.contains("A"), "END should allow output before: {out}");
    assert!(!out.contains("B"), "END should stop execution: {out}");
}

#[test]
fn basic_line_graphics() {
    let (_x, _y, _h, _pd, lines, _shapes) = run_get_turtle(Language::Basic, "LINE (0,0)-(100,100)");
    assert!(lines > 0, "LINE command should produce turtle line");
}

#[test]
fn basic_circle_graphics() {
    let (_x, _y, _h, _pd, lines, _shapes) = run_get_turtle(Language::Basic, "CIRCLE (50,50), 30");
    assert!(lines > 0, "CIRCLE should produce turtle lines: lines={lines}");
}

#[test]
fn basic_pset_graphics() {
    let (_x, _y, _h, _pd, _lines, shapes) = run_get_turtle(Language::Basic, "PSET (10,20)");
    assert!(shapes > 0, "PSET should produce a dot shape");
}

#[test]
fn basic_turtle_forward() {
    let (_x, y, _h, _pd, lines, _shapes) = run_get_turtle(Language::Basic, "FORWARD 100");
    // Forward heading 0 (north) should increase y
    assert!(y > 50.0, "FORWARD should move turtle up: y={y}");
    assert!(lines > 0, "FORWARD should draw a line");
}

#[test]
fn basic_turtle_backward() {
    let (_, y, _, _, lines, _) = run_get_turtle(Language::Basic, "BACKWARD 100");
    assert!(y < -50.0, "BACKWARD should move turtle down: y={y}");
    assert!(lines > 0, "BACKWARD should draw a line");
}

#[test]
fn basic_turtle_right_left() {
    let (_, _, h, _, _, _) = run_get_turtle(Language::Basic, "RIGHT 90");
    assert!((h - 90.0).abs() < 0.01, "RIGHT 90 should set heading to 90: h={h}");
}

#[test]
fn basic_turtle_penup_pendown() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Basic, "PENUP\nFORWARD 100\nPENDOWN\nFORWARD 100");
    assert_eq!(lines, 1, "Should draw only 1 line (pen was up for first move): lines={lines}");
}

#[test]
fn basic_turtle_home() {
    let (x, y, h, _, _, _) = run_get_turtle(Language::Basic, "FORWARD 100\nHOME");
    assert!(x.abs() < 0.01 && y.abs() < 0.01, "HOME should return to origin: x={x}, y={y}");
    assert!(h.abs() < 0.01, "HOME should reset heading: h={h}");
}

#[test]
fn basic_gpio_pinmode() {
    let out = run_program(Language::Basic, "PINMODE 4, OUTPUT\nPRINT \"OK\"");
    assert!(out.contains("OK"), "PINMODE should work: {out}");
}

#[test]
fn basic_gpio_digitalwrite() {
    let out = run_program(Language::Basic, "PINMODE 4, OUTPUT\nDIGITALWRITE 4, 1\nPRINT \"OK\"");
    assert!(out.contains("OK"), "DIGITALWRITE should work: {out}");
}

#[test]
fn basic_elseif() {
    let out = run_program(Language::Basic, "X = 7\nIF X > 10 THEN\nPRINT \"BIG\"\nELSEIF X > 5 THEN\nPRINT \"MED\"\nELSE\nPRINT \"SMALL\"\nEND IF");
    assert!(out.contains("MED"), "ELSEIF failed: {out}");
}

#[test]
fn basic_randomize() {
    assert!(run_finishes(Language::Basic, "RANDOMIZE\nPRINT RND"));
}

#[test]
fn basic_sleep_wait() {
    assert!(run_finishes(Language::Basic, "SLEEP 1\nWAIT 1\nPRINT \"OK\""));
}

#[test]
fn basic_draw_command() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Basic, "DRAW \"U10R10D10L10\"");
    assert!(lines > 0, "DRAW should produce lines: lines={lines}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOGO INTERPRETER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn logo_forward() {
    let (_, y, _, _, lines, _) = run_get_turtle(Language::Logo, "FD 100");
    assert!(y > 50.0, "FD should move up: y={y}");
    assert!(lines > 0, "FD should draw");
}

#[test]
fn logo_back() {
    let (_, y, _, _, lines, _) = run_get_turtle(Language::Logo, "BK 100");
    assert!(y < -50.0, "BK should move down: y={y}");
    assert!(lines > 0, "BK should draw");
}

#[test]
fn logo_right_left() {
    let (_, _, h, _, _, _) = run_get_turtle(Language::Logo, "RT 90");
    assert!((h - 90.0).abs() < 0.01, "RT 90 failed: h={h}");
    
    let (_, _, h2, _, _, _) = run_get_turtle(Language::Logo, "LT 45");
    assert!((h2 - 315.0).abs() < 0.01 || (h2 - (-45.0f64).rem_euclid(360.0)).abs() < 0.01,
        "LT 45 failed: h={h2}");
}

#[test]
fn logo_penup_pendown() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "PU FD 50 PD FD 50");
    assert_eq!(lines, 1, "Should have 1 line after PU FD PD FD: lines={lines}");
}

#[test]
fn logo_home() {
    let (x, y, h, _, _, _) = run_get_turtle(Language::Logo, "FD 100 HOME");
    assert!(x.abs() < 0.01 && y.abs() < 0.01, "HOME failed: x={x},y={y}");
    assert!(h.abs() < 0.01, "HOME heading failed: h={h}");
}

#[test]
fn logo_clearscreen() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "FD 100 CS FD 50");
    assert!(lines <= 1, "CS should clear previous lines: lines={lines}");
}

#[test]
fn logo_hideturtle_showturtle() {
    let (_, _, _, _, _, _) = run_get_turtle(Language::Logo, "HT");
    // Just verifying it doesn't crash
    assert!(run_finishes(Language::Logo, "HT\nST"));
}

#[test]
fn logo_setxy() {
    let (x, y, _, _, lines, _) = run_get_turtle(Language::Logo, "SETXY 100 200");
    assert!((x - 100.0).abs() < 0.01, "SETXY x failed: x={x}");
    assert!((y - 200.0).abs() < 0.01, "SETXY y failed: y={y}");
    assert!(lines > 0, "SETXY should draw line with pen down");
}

#[test]
fn logo_setx_sety() {
    let (x, _, _, _, _, _) = run_get_turtle(Language::Logo, "SETX 50");
    assert!((x - 50.0).abs() < 0.01, "SETX failed: x={x}");
    
    let (_, y, _, _, _, _) = run_get_turtle(Language::Logo, "SETY 75");
    assert!((y - 75.0).abs() < 0.01, "SETY failed: y={y}");
}

#[test]
fn logo_setheading() {
    let (_, _, h, _, _, _) = run_get_turtle(Language::Logo, "SETH 180");
    assert!((h - 180.0).abs() < 0.01, "SETH failed: h={h}");
}

#[test]
fn logo_setpencolor_name() {
    assert!(run_finishes(Language::Logo, "SETPC \"RED\nFD 50"));
}

#[test]
fn logo_setpencolor_rgb() {
    assert!(run_finishes(Language::Logo, "SETPC [255 0 0]\nFD 50"));
}

#[test]
fn logo_setpenwidth() {
    assert!(run_finishes(Language::Logo, "SETPW 5\nFD 50"));
}

#[test]
fn logo_setbgcolor() {
    assert!(run_finishes(Language::Logo, "SETBG \"BLUE"));
}

#[test]
fn logo_arc() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, "ARC 50 360");
    assert!(shapes > 0, "ARC should produce shape: shapes={shapes}");
}

#[test]
fn logo_dot() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, "DOT 10");
    assert!(shapes > 0, "DOT should produce shape: shapes={shapes}");
}

#[test]
fn logo_label() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, "LABEL \"HELLO");
    assert!(shapes > 0, "LABEL should produce text shape: shapes={shapes}");
}

#[test]
fn logo_beginfill_endfill() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, "BEGINFILL FD 50 RT 120 FD 50 RT 120 FD 50 ENDFILL");
    assert!(shapes > 0, "Fill should produce polygon shape: shapes={shapes}");
}

#[test]
fn logo_repeat() {
    let out = run_program(Language::Logo, "REPEAT 3 [PRINT \"HI]");
    let count = out.matches("HI").count();
    assert_eq!(count, 3, "REPEAT 3 should print 3 times: got {count}, out={out}");
}

#[test]
fn logo_repeat_nested() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "REPEAT 4 [FD 50 RT 90]");
    assert_eq!(lines, 4, "REPEAT 4 [FD RT] should draw 4 lines: lines={lines}");
}

#[test]
fn logo_if_true() {
    let out = run_program(Language::Logo, "MAKE \"X 5\nIF :X > 3 [PRINT \"YES]");
    assert!(out.contains("YES"), "Logo IF true failed: {out}");
}

#[test]
fn logo_if_false() {
    let out = run_program(Language::Logo, "MAKE \"X 1\nIF :X > 3 [PRINT \"YES]");
    assert!(!out.contains("YES"), "Logo IF false should not print: {out}");
}

#[test]
fn logo_ifelse() {
    let out = run_program(Language::Logo, "MAKE \"X 1\nIFELSE :X > 3 [PRINT \"BIG] [PRINT \"SMALL]");
    assert!(out.contains("SMALL"), "IFELSE false branch failed: {out}");
}

#[test]
fn logo_make_variable() {
    let out = run_program(Language::Logo, "MAKE \"X 42\nPRINT :X");
    assert!(out.contains("42"), "MAKE variable failed: {out}");
}

#[test]
fn logo_to_procedure() {
    let out = run_program(Language::Logo, "TO GREET\nPRINT \"HELLO\nEND\nGREET");
    assert!(out.contains("HELLO"), "TO procedure failed: {out}");
}

#[test]
fn logo_to_with_params() {
    let out = run_program(Language::Logo, "TO SQUARE :SIZE\nREPEAT 4 [FD :SIZE RT 90]\nEND\nSQUARE 50\nPRINT \"DONE");
    assert!(out.contains("DONE"), "TO with params failed: {out}");
}

#[test]
fn logo_print_expression() {
    let out = run_program(Language::Logo, "PRINT 3 + 4");
    assert!(out.contains("7"), "Logo PRINT expression failed: {out}");
}

#[test]
fn logo_stop() {
    let out = run_program(Language::Logo, "TO TEST\nPRINT \"A\nSTOP\nPRINT \"B\nEND\nTEST\nPRINT \"C");
    assert!(out.contains("A"), "STOP should allow output before: {out}");
    assert!(!out.contains("B"), "STOP should prevent output after: {out}");
    assert!(out.contains("C"), "Execution should continue after procedure: {out}");
}

#[test]
fn logo_forever_terminates() {
    // FOREVER should hit iteration limit and not hang
    assert!(run_finishes(Language::Logo, "FOREVER [FD 1 RT 1]"));
}

#[test]
fn logo_gpio() {
    assert!(run_finishes(Language::Logo, "PINMODE 4 \"OUTPUT\nDIGITALWRITE 4 1\nPRINT \"OK"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// PASCAL INTERPRETER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pascal_writeln() {
    let out = run_program(Language::Pascal, "program test;\nbegin\n  writeln('Hello World');\nend.");
    assert!(out.contains("Hello World"), "WRITELN failed: {out}");
}

#[test]
fn pascal_write() {
    let out = run_program(Language::Pascal, "program test;\nbegin\n  write('A');\n  write('B');\nend.");
    assert!(out.contains("A") && out.contains("B"), "WRITE failed: {out}");
}

#[test]
fn pascal_var_declaration() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  x := 42;\n  writeln(x);\nend.");
    assert!(out.contains("42"), "VAR declaration failed: {out}");
}

#[test]
fn pascal_const_declaration() {
    let out = run_program(Language::Pascal, "program test;\nconst\n  PI = 3;\nbegin\n  writeln(PI);\nend.");
    assert!(out.contains("3"), "CONST declaration failed: {out}");
}

#[test]
fn pascal_assignment() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  x := 10;\n  writeln(x);\nend.");
    assert!(out.contains("10"), "Assignment failed: {out}");
}

#[test]
fn pascal_if_then() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  x := 5;\n  if x > 3 then\n    writeln('YES');\nend.");
    assert!(out.contains("YES"), "IF/THEN failed: {out}");
}

#[test]
fn pascal_if_then_else() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  x := 1;\n  if x > 3 then writeln('BIG') else writeln('SMALL');\nend.");
    assert!(out.contains("SMALL"), "IF/THEN/ELSE failed: {out}");
}

#[test]
fn pascal_while_do() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  x := 0;\n  while x < 5 do\n  begin\n    x := x + 1;\n  end;\n  writeln(x);\nend.");
    assert!(out.contains("5"), "WHILE/DO failed: {out}");
}

#[test]
fn pascal_while_false_skip() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  x := 10;\n  while x < 5 do\n  begin\n    x := x + 1;\n  end;\n  writeln(x);\nend.");
    assert!(out.contains("10"), "WHILE false should skip body: {out}");
}

#[test]
fn pascal_for_to() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  i : integer;\nbegin\n  for i := 1 to 5 do\n    writeln(i);\nend.");
    assert!(out.contains("1") && out.contains("5"), "FOR/TO failed: {out}");
}

#[test]
fn pascal_for_downto() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  i : integer;\nbegin\n  for i := 5 downto 1 do\n    writeln(i);\nend.");
    assert!(out.contains("5") && out.contains("1"), "FOR/DOWNTO failed: {out}");
}

#[test]
fn pascal_for_begin_end() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  i, s : integer;\nbegin\n  s := 0;\n  for i := 1 to 10 do\n  begin\n    s := s + i;\n  end;\n  writeln(s);\nend.");
    assert!(out.contains("55"), "FOR with BEGIN/END failed: {out}");
}

#[test]
fn pascal_repeat_until() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  x := 0;\n  repeat\n    x := x + 1;\n  until x >= 5;\n  writeln(x);\nend.");
    assert!(out.contains("5"), "REPEAT/UNTIL failed: {out}");
}

#[test]
fn pascal_procedure() {
    let out = run_program(Language::Pascal, "program test;\nprocedure greet;\nbegin\n  writeln('Hello');\nend;\nbegin\n  greet;\nend.");
    assert!(out.contains("Hello"), "PROCEDURE failed: {out}");
}

#[test]
fn pascal_procedure_params() {
    let out = run_program(Language::Pascal, "program test;\nprocedure show(x : integer);\nbegin\n  writeln(x);\nend;\nbegin\n  show(42);\nend.");
    assert!(out.contains("42"), "PROCEDURE with params failed: {out}");
}

#[test]
fn pascal_function() {
    let out = run_program(Language::Pascal, "program test;\nfunction double(x : integer) : integer;\nbegin\n  double := x * 2;\nend;\nbegin\n  writeln(double(7));\nend.");
    assert!(out.contains("14"), "FUNCTION failed: {out}");
}

#[test]
fn pascal_readln() {
    let out = run_with_inputs(Language::Pascal, "program test;\nvar\n  x : integer;\nbegin\n  readln(x);\n  writeln(x);\nend.", &["99"]);
    assert!(out.contains("99"), "READLN failed: {out}");
}

#[test]
fn pascal_string_var() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  s : string;\nbegin\n  s := 'Hello';\n  writeln(s);\nend.");
    assert!(out.contains("Hello"), "String variable failed: {out}");
}

#[test]
fn pascal_comments() {
    let out = run_program(Language::Pascal, "program test;\n// line comment\n{ block comment }\n(* another comment *)\nbegin\n  writeln('OK');\nend.");
    assert!(out.contains("OK"), "Comments failed: {out}");
}

#[test]
fn pascal_clrscr() {
    assert!(run_finishes(Language::Pascal, "program test;\nbegin\n  clrscr;\n  writeln('OK');\nend."));
}

#[test]
fn pascal_array() {
    let out = run_program(Language::Pascal, "program test;\nvar\n  a : array[1..5] of integer;\nbegin\n  a[3] := 99;\n  writeln(a[3]);\nend.");
    assert!(out.contains("99"), "Array failed: {out}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// C LANGUAGE INTERPRETER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn c_printf_string() {
    let out = run_program(Language::C, "#include <stdio.h>\nint main() {\n  printf(\"Hello World\\n\");\n  return 0;\n}");
    assert!(out.contains("Hello World"), "printf string failed: {out}");
}

#[test]
fn c_printf_int() {
    let out = run_program(Language::C, "#include <stdio.h>\nint main() {\n  int x = 42;\n  printf(\"%d\\n\", x);\n  return 0;\n}");
    assert!(out.contains("42"), "printf %d failed: {out}");
}

#[test]
fn c_printf_float() {
    let out = run_program(Language::C, "#include <stdio.h>\nint main() {\n  printf(\"%f\\n\", 3.14);\n  return 0;\n}");
    assert!(out.contains("3.14"), "printf %f failed: {out}");
}

#[test]
fn c_printf_multiple_args() {
    let out = run_program(Language::C, "#include <stdio.h>\nint main() {\n  int a = 10;\n  int b = 20;\n  printf(\"%d + %d = %d\\n\", a, b, a + b);\n  return 0;\n}");
    assert!(out.contains("10 + 20 = 30"), "printf multiple args failed: {out}");
}

#[test]
fn c_printf_escape_sequences() {
    let out = run_program(Language::C, "#include <stdio.h>\nint main() {\n  printf(\"A\\tB\\n\");\n  return 0;\n}");
    assert!(out.contains("A\tB"), "printf escape sequences failed: {out}");
}

#[test]
fn c_scanf() {
    let out = run_with_inputs(Language::C, "#include <stdio.h>\nint main() {\n  int x;\n  scanf(\"%d\", &x);\n  printf(\"%d\\n\", x);\n  return 0;\n}", &["99"]);
    assert!(out.contains("99"), "scanf failed: {out}");
}

#[test]
fn c_variable_declaration() {
    let out = run_program(Language::C, "int main() {\n  int a = 5;\n  float b = 3.14;\n  printf(\"%d\\n\", a);\n  return 0;\n}");
    assert!(out.contains("5"), "Variable declaration failed: {out}");
}

#[test]
fn c_if_true() {
    let out = run_program(Language::C, "int main() {\n  int x = 10;\n  if (x > 5) {\n    printf(\"YES\\n\");\n  }\n  return 0;\n}");
    assert!(out.contains("YES"), "C if true failed: {out}");
}

#[test]
fn c_if_false() {
    let out = run_program(Language::C, "int main() {\n  int x = 1;\n  if (x > 5) {\n    printf(\"YES\\n\");\n  }\n  printf(\"DONE\\n\");\n  return 0;\n}");
    assert!(!out.contains("YES"), "C if false should skip: {out}");
    assert!(out.contains("DONE"), "Should continue after if: {out}");
}

#[test]
fn c_if_else() {
    let out = run_program(Language::C, "int main() {\n  int x = 1;\n  if (x > 5) {\n    printf(\"BIG\\n\");\n  } else {\n    printf(\"SMALL\\n\");\n  }\n  return 0;\n}");
    assert!(out.contains("SMALL"), "C if/else failed: {out}");
}

#[test]
fn c_if_else_if() {
    let out = run_program(Language::C, "int main() {\n  int x = 7;\n  if (x > 10) {\n    printf(\"BIG\\n\");\n  } else if (x > 5) {\n    printf(\"MED\\n\");\n  } else {\n    printf(\"SMALL\\n\");\n  }\n  return 0;\n}");
    assert!(out.contains("MED"), "C if/else if failed: {out}");
}

#[test]
fn c_inline_if() {
    let out = run_program(Language::C, "int main() {\n  int x = 1;\n  if (x <= 1) printf(\"ONE\\n\");\n  return 0;\n}");
    assert!(out.contains("ONE"), "C inline if failed: {out}");
}

#[test]
fn c_while_loop() {
    let out = run_program(Language::C, "int main() {\n  int x = 0;\n  while (x < 5) {\n    x++;\n  }\n  printf(\"%d\\n\", x);\n  return 0;\n}");
    assert!(out.contains("5"), "C while loop failed: {out}");
}

#[test]
fn c_for_loop() {
    let out = run_program(Language::C, "int main() {\n  int sum = 0;\n  for (int i = 1; i <= 10; i++) {\n    sum += i;\n  }\n  printf(\"%d\\n\", sum);\n  return 0;\n}");
    assert!(out.contains("55"), "C for loop failed: {out}");
}

#[test]
fn c_function_definition() {
    let out = run_program(Language::C, "int double_it(int x) {\n  return x * 2;\n}\nint main() {\n  printf(\"%d\\n\", double_it(7));\n  return 0;\n}");
    assert!(out.contains("14"), "C function definition failed: {out}");
}

#[test]
fn c_recursive_function() {
    let out = run_program(Language::C, "int factorial(int n) {\n  if (n <= 1) return 1;\n  return n * factorial(n - 1);\n}\nint main() {\n  printf(\"%d\\n\", factorial(5));\n  return 0;\n}");
    assert!(out.contains("120"), "C recursive function failed: {out}");
}

#[test]
fn c_increment_decrement() {
    let out = run_program(Language::C, "int main() {\n  int x = 5;\n  x++;\n  printf(\"%d\\n\", x);\n  x--;\n  printf(\"%d\\n\", x);\n  return 0;\n}");
    assert!(out.contains("6"), "Increment failed: {out}");
    assert!(out.contains("5"), "Decrement failed: {out}");
}

#[test]
fn c_compound_assign() {
    let out = run_program(Language::C, "int main() {\n  int x = 10;\n  x += 5;\n  printf(\"%d\\n\", x);\n  x -= 3;\n  printf(\"%d\\n\", x);\n  x *= 2;\n  printf(\"%d\\n\", x);\n  x /= 3;\n  printf(\"%d\\n\", x);\n  return 0;\n}");
    assert!(out.contains("15"), "+= failed: {out}");
    assert!(out.contains("12"), "-= failed: {out}");
    assert!(out.contains("24"), "*= failed: {out}");
    assert!(out.contains("8"), "/= failed: {out}");
}

#[test]
fn c_modulo_assign() {
    let out = run_program(Language::C, "int main() {\n  int x = 17;\n  x %= 5;\n  printf(\"%d\\n\", x);\n  return 0;\n}");
    assert!(out.contains("2"), "%= failed: {out}");
}

#[test]
fn c_builtin_math() {
    let out = run_program(Language::C, "#include <math.h>\nint main() {\n  printf(\"%f\\n\", sqrt(144));\n  printf(\"%d\\n\", abs(-42));\n  printf(\"%f\\n\", pow(2, 8));\n  return 0;\n}");
    assert!(out.contains("12"), "sqrt failed: {out}");
    assert!(out.contains("42"), "abs failed: {out}");
    assert!(out.contains("256"), "pow failed: {out}");
}

#[test]
fn c_return_stops_main() {
    let out = run_program(Language::C, "int main() {\n  printf(\"BEFORE\\n\");\n  return 0;\n  printf(\"AFTER\\n\");\n}");
    assert!(out.contains("BEFORE"), "Output before return: {out}");
    // After return in main, execution should stop
}

#[test]
fn c_preprocessor_ignored() {
    let out = run_program(Language::C, "#include <stdio.h>\n#define FOO 42\nint main() {\n  printf(\"OK\\n\");\n  return 0;\n}");
    assert!(out.contains("OK"), "Preprocessor should be ignored: {out}");
}

#[test]
fn c_multiple_declarations() {
    let out = run_program(Language::C, "int main() {\n  int a = 1, b = 2, c = 3;\n  printf(\"%d %d %d\\n\", a, b, c);\n  return 0;\n}");
    assert!(out.contains("1") && out.contains("2") && out.contains("3"), "Multiple decl failed: {out}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// FORTH INTERPRETER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn forth_arithmetic() {
    let out = run_program(Language::Forth, "3 4 + .\n10 3 - .\n6 7 * .\n20 5 / .");
    assert!(out.contains("7"), "3+4 failed: {out}");
    assert!(out.contains("42"), "6*7 failed: {out}");
    assert!(out.contains("4"), "20/5 failed: {out}");
}

#[test]
fn forth_mod() {
    let out = run_program(Language::Forth, "17 5 MOD .");
    assert!(out.contains("2"), "MOD failed: {out}");
}

#[test]
fn forth_divmod() {
    let out = run_program(Language::Forth, "17 5 /MOD . .");
    // /MOD pushes (remainder quotient), so . prints quotient first, then remainder
    assert!(out.contains("3"), "/MOD quotient failed: {out}");
    assert!(out.contains("2"), "/MOD remainder failed: {out}");
}

#[test]
fn forth_negate_abs() {
    let out = run_program(Language::Forth, "5 NEGATE .\n-3 ABS .");
    assert!(out.contains("-5"), "NEGATE failed: {out}");
    assert!(out.contains("3"), "ABS failed: {out}");
}

#[test]
fn forth_stack_ops() {
    let out = run_program(Language::Forth, "5 DUP . .\n3 4 SWAP . .\n1 2 OVER . . .");
    // DUP: 5 5 -> print 5 5
    // SWAP: 4 3 -> print 3 4
    // OVER: 1 2 1 -> print 1 2 1
    assert!(out.contains("5"), "DUP failed: {out}");
}

#[test]
fn forth_drop() {
    let out = run_program(Language::Forth, "1 2 3 DROP . .");
    // After DROP: 1 2 -> print 2 1
    assert!(out.contains("2"), "DROP failed: {out}");
}

#[test]
fn forth_rot() {
    let out = run_program(Language::Forth, "1 2 3 ROT . . .");
    // ROT: a b c -> b c a, so print a=1 c=3 b=2 ? No: ROT moves third to top: 2 3 1
    // . prints 1, . prints 3, . prints 2
    assert!(out.contains("1"), "ROT failed: {out}");
}

#[test]
fn forth_comparison() {
    let out = run_program(Language::Forth, "5 3 > .\n3 5 < .\n5 5 = .\n5 3 <> .");
    assert!(out.contains("-1"), "Comparison true should be -1: {out}");
}

#[test]
fn forth_logic() {
    let out = run_program(Language::Forth, "-1 -1 AND .\n0 -1 OR .\n0 INVERT .");
    assert!(out.contains("-1"), "Logic failed: {out}");
}

#[test]
fn forth_io_dot_s() {
    let out = run_program(Language::Forth, "1 2 3 .S");
    assert!(out.contains("1") && out.contains("2") && out.contains("3"), ".S failed: {out}");
}

#[test]
fn forth_cr() {
    let out = run_program(Language::Forth, ".\" Hello\" CR .\" World\"");
    assert!(out.contains("Hello") && out.contains("World"), "CR failed: {out}");
}

#[test]
fn forth_emit() {
    let out = run_program(Language::Forth, "65 EMIT");
    assert!(out.contains("A"), "EMIT 65 should print A: {out}");
}

#[test]
fn forth_space_spaces() {
    let out = run_program(Language::Forth, ".\" A\" SPACE .\" B\"");
    assert!(out.contains("A B"), "SPACE failed: {out}");
}

#[test]
fn forth_dot_quote() {
    let out = run_program(Language::Forth, ".\" Hello World\"");
    assert!(out.contains("Hello World"), "Dot-quote failed: {out}");
}

#[test]
fn forth_if_then() {
    let out = run_program(Language::Forth, "-1 IF .\" YES\" THEN");
    assert!(out.contains("YES"), "Forth IF/THEN true failed: {out}");
}

#[test]
fn forth_if_else_then_true() {
    let out = run_program(Language::Forth, "-1 IF .\" YES\" ELSE .\" NO\" THEN");
    assert!(out.contains("YES"), "IF/ELSE/THEN true failed: {out}");
    assert!(!out.contains("NO"), "Should not contain NO: {out}");
}

#[test]
fn forth_if_else_then_false() {
    let out = run_program(Language::Forth, "0 IF .\" YES\" ELSE .\" NO\" THEN");
    assert!(out.contains("NO"), "IF/ELSE/THEN false failed: {out}");
    assert!(!out.contains("YES"), "Should not contain YES: {out}");
}

#[test]
fn forth_do_loop() {
    let out = run_program(Language::Forth, "5 0 DO I . LOOP");
    assert!(out.contains("0") && out.contains("4"), "DO/LOOP failed: {out}");
}

#[test]
fn forth_do_plus_loop() {
    let out = run_program(Language::Forth, "10 0 DO I . 2 +LOOP");
    assert!(out.contains("0") && out.contains("8"), "DO/+LOOP failed: {out}");
}

#[test]
fn forth_nested_do_loop_j() {
    let out = run_program(Language::Forth, "3 0 DO 3 0 DO I . LOOP LOOP");
    // Inner loop runs 3 times per outer iteration, outer runs 3 times = 9 numbers
    assert!(out.contains("0"), "Nested DO/LOOP failed: {out}");
}

#[test]
fn forth_begin_until() {
    let out = run_program(Language::Forth, "0 BEGIN DUP . 1+ DUP 5 >= UNTIL DROP");
    assert!(out.contains("0") && out.contains("4"), "BEGIN/UNTIL failed: {out}");
}

#[test]
fn forth_begin_again_terminates() {
    // Should hit iteration limit and not hang
    assert!(run_finishes(Language::Forth, "BEGIN 1 AGAIN"));
}

#[test]
fn forth_begin_while_repeat() {
    let out = run_program(Language::Forth, "0 BEGIN DUP 5 < WHILE DUP . 1+ REPEAT DROP");
    assert!(out.contains("0") && out.contains("4"), "BEGIN/WHILE/REPEAT failed: {out}");
}

#[test]
fn forth_define_word() {
    let out = run_program(Language::Forth, ": DOUBLE 2 * ;\n5 DOUBLE .");
    assert!(out.contains("10"), "User-defined word failed: {out}");
}

#[test]
fn forth_variable() {
    let out = run_program(Language::Forth, "VARIABLE X\n42 X !\nX @ .");
    assert!(out.contains("42"), "VARIABLE/!/@ failed: {out}");
}

#[test]
fn forth_constant() {
    let out = run_program(Language::Forth, "99 CONSTANT LIMIT\nLIMIT .");
    assert!(out.contains("99"), "CONSTANT failed: {out}");
}

#[test]
fn forth_memory_operations() {
    let out = run_program(Language::Forth, "42 0 !\n0 @ .\n7 0 +!\n0 @ .");
    // Store 42 at addr 0, fetch -> 42, add 7 -> 49
    assert!(out.contains("42"), "Memory ! @ failed: {out}");
    assert!(out.contains("49"), "Memory +! failed: {out}");
}

#[test]
fn forth_leave() {
    let out = run_program(Language::Forth, "10 0 DO I DUP 3 = IF LEAVE THEN . LOOP .\" DONE\"");
    assert!(out.contains("DONE"), "LEAVE should exit loop: {out}");
    // Should print 0 1 2 then LEAVE
    assert!(out.contains("0") && out.contains("1") && out.contains("2"), "LEAVE should print 0-2: {out}");
}

#[test]
fn forth_1plus_1minus() {
    let out = run_program(Language::Forth, "5 1+ .\n5 1- .");
    assert!(out.contains("6"), "1+ failed: {out}");
    assert!(out.contains("4"), "1- failed: {out}");
}

#[test]
fn forth_2star_2slash() {
    let out = run_program(Language::Forth, "5 2* .\n10 2/ .");
    assert!(out.contains("10"), "2* failed: {out}");
    assert!(out.contains("5"), "2/ failed: {out}");
}

#[test]
fn forth_max_min() {
    let out = run_program(Language::Forth, "3 7 MAX .\n3 7 MIN .");
    assert!(out.contains("7"), "MAX failed: {out}");
    assert!(out.contains("3"), "MIN failed: {out}");
}

#[test]
fn forth_hex_literal() {
    let out = run_program(Language::Forth, "0xFF .");
    assert!(out.contains("255"), "Hex literal failed: {out}");
}

#[test]
fn forth_0_equals() {
    let out = run_program(Language::Forth, "0 0= .\n5 0= .");
    // 0= on 0 should give -1 (true), on 5 should give 0
    assert!(out.contains("-1"), "0= on 0 failed: {out}");
    assert!(out.contains("0"), "0= on 5 failed: {out}");
}

#[test]
fn forth_turtle_fd_rt() {
    let (_, y, _, _, lines, _) = run_get_turtle(Language::Forth, "100 FD");
    assert!(y > 50.0, "Forth FD should move up: y={y}");
    assert!(lines > 0, "Forth FD should draw");
}

#[test]
fn forth_turtle_penup_down() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Forth, "PU 100 FD PD 100 FD");
    assert_eq!(lines, 1, "Forth PU/PD should draw only 1 line: lines={lines}");
}

#[test]
fn forth_comments() {
    let out = run_program(Language::Forth, "\\ This is a comment\n( This too ) 5 .");
    assert!(out.contains("5"), "Forth comments failed: {out}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// PILOT INTERPRETER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pilot_type() {
    let out = run_program(Language::Pilot, "T:Hello World");
    assert!(out.contains("Hello World"), "T: failed: {out}");
}

#[test]
fn pilot_compute() {
    let out = run_program(Language::Pilot, "C:X = 42\nT:#X#");
    assert!(out.contains("42"), "C:/T: variable failed: {out}");
}

#[test]
fn pilot_accept_and_match() {
    let out = run_with_inputs(Language::Pilot, "T:Enter name\nA:\nM:JAMES\nTY:Match!\nTN:No match", &["JAMES"]);
    assert!(out.contains("Match"), "A:/M:/TY: failed: {out}");
}

#[test]
fn pilot_match_no() {
    let out = run_with_inputs(Language::Pilot, "T:Enter\nA:\nM:BOB\nTY:Yes\nTN:No", &["ALICE"]);
    assert!(out.contains("No"), "TN: on mismatch failed: {out}");
}

#[test]
fn pilot_jump() {
    let out = run_program(Language::Pilot, "J:SKIP\nT:BAD\n*SKIP\nT:GOOD");
    assert!(out.contains("GOOD"), "J: jump failed: {out}");
    assert!(!out.contains("BAD"), "J: didn't skip: {out}");
}

#[test]
fn pilot_end() {
    let out = run_program(Language::Pilot, "T:BEFORE\nE:\nT:AFTER");
    assert!(out.contains("BEFORE"), "E: should allow output before: {out}");
    assert!(!out.contains("AFTER"), "E: should stop execution: {out}");
}

#[test]
fn pilot_conditional_y_n() {
    let out = run_with_inputs(Language::Pilot, "T:Enter\nA:\nM:YES\nJY:GOOD\nJ:BAD\n*GOOD\nT:Success\nE:\n*BAD\nT:Failed", &["YES"]);
    assert!(out.contains("Success"), "JY: conditional jump failed: {out}");
}

#[test]
fn pilot_verbose_print() {
    let out = run_program(Language::Pilot, "PRINT Hello World");
    assert!(out.contains("Hello World"), "PRINT verbose failed: {out}");
}

#[test]
fn pilot_verbose_compute() {
    let out = run_program(Language::Pilot, "COMPUTE X 5 + 3\nPRINT $X");
    assert!(out.contains("8"), "COMPUTE verbose failed: {out}");
}

#[test]
fn pilot_verbose_jump() {
    let out = run_program(Language::Pilot, "JUMP DONE\nPRINT BAD\n*DONE\nPRINT GOOD");
    assert!(out.contains("GOOD"), "JUMP verbose failed: {out}");
}

#[test]
fn pilot_subroutine() {
    let out = run_program(Language::Pilot, "TU GREET\nT:Back\nE:\n*GREET\nT:Hello\nRETURN");
    assert!(out.contains("Hello"), "TU subroutine failed: {out}");
    assert!(out.contains("Back"), "RETURN failed: {out}");
}

#[test]
fn pilot_match_case_block() {
    let out = run_with_inputs(Language::Pilot, "T:Enter number\nA:NUM\nMATCH $NUM\nCASE 1:\nT:One\nCASE 2:\nT:Two\nDEFAULT:\nT:Other\nEND", &["2"]);
    assert!(out.contains("Two"), "MATCH/CASE block failed: {out}");
}

#[test]
fn pilot_display() {
    let out = run_program(Language::Pilot, "D:Hello");
    assert!(out.contains("Hello"), "D: display failed: {out}");
}

#[test]
fn pilot_remark() {
    let out = run_program(Language::Pilot, "R:This is a comment\nT:OK");
    assert!(out.contains("OK"), "R: remark failed: {out}");
}

#[test]
fn pilot_hash_interpolation() {
    let out = run_program(Language::Pilot, "C:X = 10\nT:Value is #X#");
    assert!(out.contains("10"), "#VAR# interpolation failed: {out}");
}

#[test]
fn pilot_rule_and_tu() {
    let out = run_program(Language::Pilot, "TU MYSUB\nT:After\nE:\nRULE MYSUB\nT:Inside\nRETURN");
    assert!(out.contains("Inside"), "RULE/TU failed: {out}");
    assert!(out.contains("After"), "RETURN after TU failed: {out}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROLOG INTERPRETER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn prolog_fact_and_query() {
    let out = run_program(Language::Prolog, "parent(tom, bob).\nparent(tom, liz).\n?- parent(tom, bob).");
    assert!(out.contains("true") || out.contains("yes") || out.contains("bob"), "Prolog fact query failed: {out}");
}

#[test]
fn prolog_rule_and_query() {
    let out = run_program(Language::Prolog, "parent(tom, bob).\nparent(bob, ann).\ngrandparent(X, Z) :- parent(X, Y), parent(Y, Z).\n?- grandparent(tom, ann).");
    assert!(out.contains("true") || out.contains("yes") || out.contains("ann"), "Prolog rule query failed: {out}");
}

#[test]
fn prolog_variable_binding() {
    let out = run_program(Language::Prolog, "likes(mary, food).\nlikes(mary, wine).\n?- likes(mary, X).");
    // Should show bindings for X
    assert!(out.contains("food") || out.contains("wine"), "Prolog variable binding failed: {out}");
}

#[test]
fn prolog_no_match() {
    let out = run_program(Language::Prolog, "parent(tom, bob).\n?- parent(alice, bob).");
    assert!(out.contains("false") || out.contains("no"), "Prolog no match should return false: {out}");
}

#[test]
fn prolog_multiple_solutions() {
    let out = run_program(Language::Prolog, "color(red).\ncolor(green).\ncolor(blue).\n?- color(X).");
    assert!(out.contains("red") && out.contains("green") && out.contains("blue"),
        "Prolog should find all solutions: {out}");
}

#[test]
fn prolog_write() {
    let out = run_program(Language::Prolog, "?- write(hello).");
    assert!(out.contains("hello"), "Prolog write/1 failed: {out}");
}

#[test]
fn prolog_recursive_rule() {
    let out = run_program(Language::Prolog, "parent(a, b).\nparent(b, c).\nparent(c, d).\nancestor(X, Y) :- parent(X, Y).\nancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).\n?- ancestor(a, d).");
    assert!(out.contains("true") || out.contains("yes") || out.contains("d"), "Prolog recursive rule failed: {out}");
}

#[test]
fn prolog_anonymous_variable() {
    let out = run_program(Language::Prolog, "parent(tom, bob).\nparent(tom, liz).\n?- parent(tom, _).");
    assert!(out.contains("true") || out.contains("yes") || out.contains("solution"), "Prolog anonymous variable failed: {out}");
}

#[test]
fn prolog_comments() {
    let out = run_program(Language::Prolog, "% This is a comment\nfoo(bar).\n?- foo(bar).");
    assert!(out.contains("true") || out.contains("yes") || out.contains("bar"), "Prolog comments failed: {out}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXPRESSION EVALUATOR TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn eval_basic_arithmetic() {
    let out = run_program(Language::Basic, "PRINT 2 + 3\nPRINT 10 - 4\nPRINT 6 * 7\nPRINT 20 / 4");
    assert!(out.contains("5"), "2+3 failed: {out}");
    assert!(out.contains("6"), "10-4 failed: {out}");
    assert!(out.contains("42"), "6*7 failed: {out}");
}

#[test]
fn eval_operator_precedence() {
    let out = run_program(Language::Basic, "PRINT 2 + 3 * 4");
    assert!(out.contains("14"), "Operator precedence failed: {out}");
}

#[test]
fn eval_parentheses() {
    let out = run_program(Language::Basic, "PRINT (2 + 3) * 4");
    assert!(out.contains("20"), "Parentheses failed: {out}");
}

#[test]
fn eval_power() {
    let out = run_program(Language::Basic, "PRINT 2 ^ 10");
    assert!(out.contains("1024"), "Power operator failed: {out}");
}

#[test]
fn eval_modulo() {
    let out = run_program(Language::Basic, "PRINT 17 MOD 5");
    assert!(out.contains("2"), "MOD operator failed: {out}");
}

#[test]
fn eval_comparison_ops() {
    // In BASIC, true = -1, false = 0
    let out = run_program(Language::Basic, "PRINT 5 > 3\nPRINT 3 > 5\nPRINT 5 = 5\nPRINT 5 <> 3\nPRINT 3 <= 5\nPRINT 5 >= 5");
    assert!(out.contains("-1"), "Comparison true should be -1: {out}");
    assert!(out.contains("0"), "Comparison false should be 0: {out}");
}

#[test]
fn eval_logical_and_or_not() {
    let out = run_program(Language::Basic, "PRINT (5 > 3) AND (10 > 1)");
    assert!(out.contains("-1"), "AND failed: {out}");
    
    let out2 = run_program(Language::Basic, "PRINT NOT (5 > 3)");
    assert!(out2.contains("0"), "NOT failed: {out2}");
}

#[test]
fn eval_unary_minus() {
    let out = run_program(Language::Basic, "PRINT -5 + 3");
    assert!(out.contains("-2"), "Unary minus failed: {out}");
}

#[test]
fn eval_trig_functions() {
    let out = run_program(Language::Basic, "PRINT SIN(90)\nPRINT COS(0)\nPRINT TAN(45)");
    assert!(out.contains("1"), "SIN(90) should be ~1: {out}");
}

#[test]
fn eval_sqrt_abs() {
    let out = run_program(Language::Basic, "PRINT SQRT(144)\nPRINT ABS(-42)");
    assert!(out.contains("12"), "SQRT failed: {out}");
    assert!(out.contains("42"), "ABS failed: {out}");
}

#[test]
fn eval_floor_ceil_round() {
    let out = run_program(Language::Basic, "PRINT FLOOR(3.7)\nPRINT CEIL(3.2)\nPRINT ROUND(3.5)");
    assert!(out.contains("3"), "FLOOR failed: {out}");
    assert!(out.contains("4"), "CEIL/ROUND failed: {out}");
}

#[test]
fn eval_exp_log() {
    let out = run_program(Language::Basic, "PRINT INT(EXP(1) * 100)");
    // e ≈ 2.718, *100 = 271.8, INT = 271
    assert!(out.contains("271"), "EXP failed: {out}");
}

#[test]
fn eval_log10() {
    let out = run_program(Language::Basic, "PRINT LOG10(100)");
    assert!(out.contains("2"), "LOG10 failed: {out}");
}

#[test]
fn eval_sgn() {
    let out = run_program(Language::Basic, "PRINT SGN(5)\nPRINT SGN(-3)\nPRINT SGN(0)");
    assert!(out.contains("1"), "SGN positive failed: {out}");
    assert!(out.contains("-1"), "SGN negative failed: {out}");
    assert!(out.contains("0"), "SGN zero failed: {out}");
}

#[test]
fn eval_min_max() {
    let out = run_program(Language::Basic, "PRINT MIN(3, 7)\nPRINT MAX(3, 7)");
    assert!(out.contains("3"), "MIN failed: {out}");
    assert!(out.contains("7"), "MAX failed: {out}");
}

#[test]
fn eval_pow_function() {
    let out = run_program(Language::Basic, "PRINT POW(2, 8)");
    assert!(out.contains("256"), "POW function failed: {out}");
}

#[test]
fn eval_pi_e_constants() {
    let out = run_program(Language::Basic, "PRINT INT(PI * 100)\nPRINT INT(E * 100)");
    assert!(out.contains("314"), "PI failed: {out}");
    assert!(out.contains("271"), "E failed: {out}");
}

#[test]
fn eval_rand() {
    let out = run_program(Language::Basic, "X = RND\nIF X >= 0 AND X < 1 THEN PRINT \"OK\"");
    assert!(out.contains("OK"), "RND should be 0-1: {out}");
}

#[test]
fn eval_random() {
    let out = run_program(Language::Basic, "X = RANDOM(10)\nIF X >= 0 AND X < 10 THEN PRINT \"OK\"");
    assert!(out.contains("OK"), "RANDOM should be 0-9: {out}");
}

#[test]
fn eval_nested_functions() {
    let out = run_program(Language::Basic, "PRINT ABS(MIN(-5, -10))");
    assert!(out.contains("10"), "Nested functions failed: {out}");
}

#[test]
fn eval_complex_expression() {
    let out = run_program(Language::Basic, "PRINT (2 + 3) * 4 - 1");
    assert!(out.contains("19"), "Complex expression failed: {out}");
}

#[test]
fn eval_asin_acos_atan() {
    let out = run_program(Language::Basic, "PRINT INT(ASIN(1))\nPRINT INT(ACOS(0))");
    assert!(out.contains("90"), "ASIN/ACOS failed: {out}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// TURTLE GRAPHICS TESTS (via Logo for most direct access)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn turtle_forward_draws_line() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "FD 100");
    assert_eq!(lines, 1, "FD should draw exactly 1 line");
}

#[test]
fn turtle_pen_up_no_line() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "PU FD 100");
    assert_eq!(lines, 0, "PU FD should draw 0 lines");
}

#[test]
fn turtle_heading_0_is_north() {
    let (x, y, _, _, _, _) = run_get_turtle(Language::Logo, "FD 100");
    assert!(x.abs() < 1.0, "FD at heading 0 should not change x: x={x}");
    assert!(y > 90.0, "FD at heading 0 should go north (positive y): y={y}");
}

#[test]
fn turtle_heading_90_is_east() {
    let (x, _, _, _, _, _) = run_get_turtle(Language::Logo, "RT 90 FD 100");
    assert!(x > 90.0, "FD at heading 90 should go east: x={x}");
}

#[test]
fn turtle_heading_180_is_south() {
    let (_, y, _, _, _, _) = run_get_turtle(Language::Logo, "RT 180 FD 100");
    assert!(y < -90.0, "FD at heading 180 should go south: y={y}");
}

#[test]
fn turtle_heading_270_is_west() {
    let (x, _, _, _, _, _) = run_get_turtle(Language::Logo, "RT 270 FD 100");
    assert!(x < -90.0, "FD at heading 270 should go west: x={x}");
}

#[test]
fn turtle_square() {
    let (x, y, h, _, lines, _) = run_get_turtle(Language::Logo, "REPEAT 4 [FD 100 RT 90]");
    assert_eq!(lines, 4, "Square should have 4 lines");
    assert!(x.abs() < 1.0 && y.abs() < 1.0, "Square should return to origin: x={x}, y={y}");
    assert!((h - 360.0).abs() < 1.0 || h.abs() < 1.0, "Heading should be 0/360: h={h}");
}

#[test]
fn turtle_arc_shape() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, "ARC 50 180");
    assert!(shapes > 0, "ARC should create shape");
}

#[test]
fn turtle_dot_shape() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, "DOT 5");
    assert_eq!(shapes, 1, "DOT should create 1 shape");
}

#[test]
fn turtle_label_shape() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, "LABEL \"Test");
    assert_eq!(shapes, 1, "LABEL should create 1 shape");
}

#[test]
fn turtle_fill_polygon() {
    let (_, _, _, _, _, shapes) = run_get_turtle(Language::Logo, 
        "BEGINFILL FD 100 RT 120 FD 100 RT 120 FD 100 RT 120 ENDFILL");
    let has_polygon = shapes > 0;
    assert!(has_polygon, "Fill should produce polygon shape");
}

#[test]
fn turtle_setxy_draws_line() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "SETXY 100 100");
    assert!(lines > 0, "SETXY with pen down should draw line");
}

#[test]
fn turtle_setxy_no_line_penup() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "PU SETXY 100 100");
    assert_eq!(lines, 0, "SETXY with pen up should not draw line");
}

// ═══════════════════════════════════════════════════════════════════════════════
// EDGE CASES AND REGRESSION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn basic_empty_program() {
    assert!(run_finishes(Language::Basic, ""));
}

#[test]
fn logo_empty_program() {
    assert!(run_finishes(Language::Logo, ""));
}

#[test]
fn pascal_empty_program() {
    assert!(run_finishes(Language::Pascal, ""));
}

#[test]
fn c_empty_program() {
    assert!(run_finishes(Language::C, ""));
}

#[test]
fn forth_empty_program() {
    assert!(run_finishes(Language::Forth, ""));
}

#[test]
fn pilot_empty_program() {
    assert!(run_finishes(Language::Pilot, ""));
}

#[test]
fn prolog_empty_program() {
    assert!(run_finishes(Language::Prolog, ""));
}

#[test]
fn basic_division_by_zero() {
    // Should not panic
    assert!(run_finishes(Language::Basic, "PRINT 10 / 0"));
}

#[test]
fn forth_division_by_zero() {
    assert!(run_finishes(Language::Forth, "10 0 /"));
}

#[test]
fn c_nested_for_while() {
    let out = run_program(Language::C, "int main() {\n  int count = 0;\n  for (int i = 0; i < 3; i++) {\n    int j = 0;\n    while (j < 3) {\n      count++;\n      j++;\n    }\n  }\n  printf(\"%d\\n\", count);\n  return 0;\n}");
    assert!(out.contains("9"), "Nested for/while failed: {out}");
}

#[test]
fn basic_nested_for_loops() {
    let out = run_program(Language::Basic, "S = 0\nFOR I = 1 TO 3\nFOR J = 1 TO 3\nS = S + 1\nNEXT J\nNEXT I\nPRINT S");
    assert!(out.contains("9"), "Nested FOR loops failed: {out}");
}

#[test]
fn logo_nested_repeat() {
    let (_, _, _, _, lines, _) = run_get_turtle(Language::Logo, "REPEAT 3 [REPEAT 4 [FD 10 RT 90] RT 120]");
    assert!(lines == 12, "3 squares should have 12 lines: lines={lines}");
}

#[test]
fn forth_call_depth_limit() {
    // Recursive word with no base case should hit call depth limit
    assert!(run_finishes(Language::Forth, ": BOOM BOOM ;\nBOOM"));
}

#[test]
fn pascal_for_iteration_limit() {
    // Broken FOR loop shouldn't hang
    assert!(run_finishes(Language::Pascal, "program test;\nvar\n  i : integer;\nbegin\n  for i := 1 to 1000000 do\n    writeln(i);\nend."));
}

#[test]
fn prolog_cycle_depth_limit() {
    // Cyclic fact shouldn't stack overflow
    assert!(run_finishes(Language::Prolog, "parent(a, b).\nparent(b, a).\nancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).\n?- ancestor(a, c)."));
}

#[test]
fn c_calculator_produces_output() {
    // This was the original bug report - 09_calculator.c produced no output
    let out = run_with_inputs(Language::C, 
        "int factorial(int n) {\n  if (n <= 1) return 1;\n  return n * factorial(n - 1);\n}\nint main() {\n  printf(\"Calculator\\n\");\n  return 0;\n}", 
        &[]);
    assert!(out.contains("Calculator"), "C with function definition should produce output: {out}");
}

#[test]
fn basic_line_number_goto() {
    let out = run_program(Language::Basic, "10 GOTO 30\n20 PRINT \"BAD\"\n30 PRINT \"GOOD\"");
    assert!(out.contains("GOOD"), "Line number GOTO failed: {out}");
    assert!(!out.contains("BAD"), "Should skip line 20: {out}");
}

#[test]
fn basic_interpolation() {
    let out = run_program(Language::Basic, "X = 42\nPRINT \"Value is *X*\"");
    assert!(out.contains("42"), "Variable interpolation failed: {out}");
}
