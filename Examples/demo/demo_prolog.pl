% =============================================
% Time Warp Prolog — Grand Demo
% A comprehensive Prolog showcase
% =============================================

/* ==========================================
   TIME WARP PROLOG - GRAND DEMO

   This program demonstrates every feature
   of the Time Warp Prolog interpreter.
   ========================================== */

?- write(prolog_grand_demo_start).

% =============================================
% SECTION 1: Facts (multiple arities)
% =============================================

% Arity 1
is_language(basic).
is_language(logo).
is_language(pascal).
is_language(c).
is_language(forth).
is_language(pilot).
is_language(prolog).

% Arity 2
year_created(basic, 1964).
year_created(logo, 1967).
year_created(pascal, 1970).
year_created(c, 1972).
year_created(forth, 1970).
year_created(pilot, 1962).
year_created(prolog, 1972).

% Arity 3
language_detail(basic, kemeny, beginner).
language_detail(logo, papert, beginner).
language_detail(pascal, wirth, intermediate).
language_detail(c, ritchie, advanced).
language_detail(forth, moore, advanced).
language_detail(pilot, carbonell, beginner).
language_detail(prolog, colmerauer, advanced).

% Arity 4
feature(basic, print, output, essential).
feature(logo, turtle, graphics, essential).
feature(pascal, types, safety, essential).
feature(c, pointers, memory, advanced).
feature(forth, stack, core, essential).
feature(pilot, match, pattern, essential).
feature(prolog, unify, logic, essential).

?- write(section_1_facts_loaded).

% =============================================
% SECTION 2: Queries with Variables
% =============================================

% All Time Warp languages
?- is_language(X).

% All creation years
?- year_created(Lang, Year).

% Beginner languages
?- language_detail(Lang, _, beginner).

% Advanced languages
?- language_detail(Lang, _, advanced).

% Essential features
?- feature(Lang, Feature, _, essential).

?- write(section_2_queries_done).

% =============================================
% SECTION 3: Rules (single & multi-goal)
% =============================================

% Single-goal rule
is_beginner_friendly(X) :- language_detail(X, _, beginner).

% Multi-goal rule  
created_by(Lang, Creator) :- language_detail(Lang, Creator, _).

% Rule with multiple clauses
is_seventies(X) :- year_created(X, 1970).
is_seventies(X) :- year_created(X, 1972).

is_sixties(X) :- year_created(X, 1962).
is_sixties(X) :- year_created(X, 1964).
is_sixties(X) :- year_created(X, 1967).

?- is_beginner_friendly(Lang).
?- is_seventies(Lang).
?- is_sixties(Lang).

?- write(section_3_rules_done).

% =============================================
% SECTION 4: Family Tree & Recursion
% =============================================

parent(ada, charles).
parent(ada, diana).
parent(charles, edward).
parent(charles, fiona).
parent(diana, george).
parent(diana, helen).
parent(edward, ivan).
parent(fiona, julia).
parent(george, karl).

male(charles).
male(edward).
male(george).
male(ivan).
male(karl).
female(ada).
female(diana).
female(fiona).
female(helen).
female(julia).

% Rules
father(X, Y) :- parent(X, Y), male(X).
mother(X, Y) :- parent(X, Y), female(X).
grandparent(X, Y) :- parent(X, Z), parent(Z, Y).
sibling(X, Y) :- parent(Z, X), parent(Z, Y).

% Recursive ancestor
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

% Queries
?- father(X, Y).
?- mother(X, Y).
?- grandparent(X, Y).
?- ancestor(ada, Descendant).

?- write(section_4_recursion_done).

% =============================================
% SECTION 5: Unification & Anonymous Variable
% =============================================

student(alice, 20, maths, a).
student(bob, 21, physics, b).
student(carol, 19, maths, a).
student(dave, 22, cs, c).
student(eve, 20, cs, a).
student(frank, 21, physics, b).

% Ignore age and grade — find all CS students
?- student(Name, _, cs, _).

% Ignore name and subject — find all A students
?- student(Name, _, _, a).

% All student records
?- student(Name, Age, Subject, Grade).

?- write(section_5_unification_done).

% =============================================
% SECTION 6: Backtracking (multiple paths)
% =============================================

flight(london, paris).
flight(london, amsterdam).
flight(paris, berlin).
flight(paris, rome).
flight(berlin, vienna).
flight(amsterdam, berlin).
flight(vienna, rome).
flight(rome, athens).

can_fly(X, Y) :- flight(X, Y).
can_fly(X, Y) :- flight(X, Z), can_fly(Z, Y).

% All destinations from London
?- can_fly(london, Dest).

% Can we fly London to Athens?
?- can_fly(london, athens).

% All destinations from Paris
?- can_fly(paris, Dest).

?- write(section_6_backtracking_done).

% =============================================
% SECTION 7: Knowledge Base (Expert System)
% =============================================

planet(mercury, rocky, 1, 0).
planet(venus, rocky, 2, 0).
planet(earth, rocky, 3, 1).
planet(mars, rocky, 4, 2).
planet(jupiter, gas, 5, 95).
planet(saturn, gas, 6, 146).
planet(uranus, ice, 7, 27).
planet(neptune, ice, 8, 16).

inner(X) :- planet(X, rocky, _, _).
outer(X) :- planet(X, gas, _, _).
outer(X) :- planet(X, ice, _, _).

?- inner(P).
?- outer(P).
?- planet(Name, Type, Position, Moons).

?- write(section_7_knowledge_done).

% =============================================
% SECTION 8: write() Predicate
% =============================================

?- write(hello_from_prolog).
?- write(time_warp_rules).
?- write(logic_programming_is_powerful).

?- write(section_8_write_done).

% =============================================
% SECTION 9: Complex Chained Reasoning
% =============================================

manages(ceo, cto).
manages(ceo, cfo).
manages(cto, lead_dev).
manages(cto, lead_qa).
manages(cfo, accountant).
manages(lead_dev, dev1).
manages(lead_dev, dev2).
manages(lead_qa, tester1).

reports_to(X, Y) :- manages(Y, X).
chain(X, Y) :- manages(Y, X).
chain(X, Y) :- manages(Z, X), chain(Z, Y).

% Who does dev1 report to (full chain)?
?- chain(dev1, Boss).

% Who reports to the CEO?
?- manages(ceo, Direct).

% Full org chart
?- manages(Manager, Report).

?- write(section_9_reasoning_done).

% =============================================
% SECTION 10: Multi-line Clauses
% =============================================

% Multi-line rule (dot-terminated)
qualified_developer(X) :-
    language_detail(X, _, advanced),
    is_language(X).

?- qualified_developer(X).

% Complex multi-goal
full_stack(X) :-
    is_language(X),
    feature(X, _, _, essential).

?- full_stack(X).

% =============================================
% FINALE
% =============================================

?- write(grand_demo_complete).
?- write(features_shown).
?- write(facts_rules_queries).
?- write(variables_unification_anonymous).
?- write(recursion_backtracking).
?- write(knowledge_bases).
?- write(chained_reasoning).
?- write(write_predicate).
?- write(multi_line_clauses).
?- write(end_of_prolog_demo).
