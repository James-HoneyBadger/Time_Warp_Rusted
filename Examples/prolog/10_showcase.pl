% =============================================
% Prolog Example 10 — Complete Showcase
% Time Warp Studio
% =============================================
% This showcase demonstrates every feature of
% the Time Warp Prolog interpreter:
%
%   - Facts (with multiple arities)
%   - Rules (single and multi-goal bodies)
%   - Queries (?-)
%   - Variables (uppercase) and unification
%   - Anonymous variable (_)
%   - Recursive rules
%   - Backtracking (multiple solutions)
%   - write() predicate
%   - Multi-line clauses (dot-terminated)
%   - Comments (% line and /* block */)

/* ==========================================
   SECTION 1: Simple Facts
   ========================================== */

?- write(section_1_facts).

% Arity-1 facts
programming_language(basic).
programming_language(logo).
programming_language(pascal).
programming_language(c).
programming_language(forth).
programming_language(pilot).
programming_language(prolog).

% Arity-2 facts
created_by(basic, kemeny).
created_by(logo, papert).
created_by(pascal, wirth).
created_by(c, ritchie).
created_by(forth, moore).
created_by(pilot, carbonell).
created_by(prolog, colmerauer).

% Arity-3 facts
language_info(basic, 1964, beginner).
language_info(logo, 1967, beginner).
language_info(pascal, 1970, intermediate).
language_info(c, 1972, advanced).
language_info(forth, 1970, advanced).
language_info(pilot, 1962, beginner).
language_info(prolog, 1972, advanced).

% All Time Warp languages
?- programming_language(X).

% Who created each language?
?- created_by(Lang, Creator).

% Beginner-friendly languages
?- language_info(Lang, _, beginner).

/* ==========================================
   SECTION 2: Rules
   ========================================== */

?- write(section_2_rules).

% Single-goal rules
is_retro(X) :- language_info(X, Year, _).

% Multi-goal rules
beginner_friendly(X) :-
    programming_language(X),
    language_info(X, _, beginner).

advanced_language(X) :-
    programming_language(X),
    language_info(X, _, advanced).

% What languages are beginner-friendly?
?- beginner_friendly(X).

% What languages are advanced?
?- advanced_language(X).

/* ==========================================
   SECTION 3: Family Tree with Recursion
   ========================================== */

?- write(section_3_recursion).

parent(arthur, betty).
parent(arthur, charles).
parent(betty, diana).
parent(betty, edward).
parent(charles, fiona).
parent(diana, george).
parent(edward, helen).
parent(fiona, ivan).

male(arthur).
male(charles).
male(edward).
male(george).
male(ivan).
female(betty).
female(diana).
female(fiona).
female(helen).

% Rules
father(X, Y) :- parent(X, Y), male(X).
mother(X, Y) :- parent(X, Y), female(X).
grandparent(X, Y) :- parent(X, Z), parent(Z, Y).
sibling(X, Y) :- parent(Z, X), parent(Z, Y).

% Recursive: ancestor
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

% Queries
?- father(X, Y).
?- mother(X, Y).
?- grandparent(X, Y).
?- ancestor(arthur, Descendant).

/* ==========================================
   SECTION 4: Unification & Anonymous Var
   ========================================== */

?- write(section_4_unification).

% Multi-field facts
student(alice, 20, computer_science).
student(bob, 22, mathematics).
student(carol, 19, physics).
student(dave, 21, computer_science).
student(eve, 20, mathematics).

% Using _ to ignore fields
% All student names
?- student(Name, _, _).

% All CS students
?- student(Name, _, computer_science).

% All math students
?- student(Name, _, mathematics).

% All student details
?- student(Name, Age, Major).

/* ==========================================
   SECTION 5: Backtracking
   ========================================== */

?- write(section_5_backtracking).

% Transport network
route(london, paris).
route(paris, berlin).
route(berlin, vienna).
route(vienna, rome).
route(london, amsterdam).
route(amsterdam, berlin).
route(paris, rome).

% Can travel (with backtracking for all paths)
can_travel(X, Y) :- route(X, Y).
can_travel(X, Y) :- route(X, Z), can_travel(Z, Y).

% Where can you travel from London?
?- can_travel(london, Destination).

% Can you get from London to Rome?
?- can_travel(london, rome).

% Can you get from Amsterdam to Vienna?
?- can_travel(amsterdam, vienna).

/* ==========================================
   SECTION 6: Knowledge Representation
   ========================================== */

?- write(section_6_knowledge).

% Animal classification
animal(dog, mammal, legs_4, domestic).
animal(cat, mammal, legs_4, domestic).
animal(eagle, bird, legs_2, wild).
animal(penguin, bird, legs_2, wild).
animal(salmon, fish, legs_0, wild).
animal(horse, mammal, legs_4, domestic).

% Classification rules
is_pet(X) :- animal(X, _, _, domestic).
flies(eagle).
swims(salmon).
swims(penguin).

% Queries
?- is_pet(X).
?- animal(X, bird, _, _).
?- animal(X, mammal, _, _).

/* ==========================================
   SECTION 7: write() Predicate
   ========================================== */

?- write(section_7_write).
?- write(prolog_showcase_complete).
?- write(all_features_demonstrated).

/* ==========================================
   SECTION 8: Complex Reasoning
   ========================================== */

?- write(section_8_reasoning).

% Friendship — symmetric relationship
friend_of(alice, bob).
friend_of(bob, carol).
friend_of(carol, dave).
friend_of(alice, eve).
friend_of(eve, frank).

% Friend-of-a-friend
foaf(X, Y) :- friend_of(X, Z), friend_of(Z, Y).

% Social chain
knows(X, Y) :- friend_of(X, Y).
knows(X, Y) :- friend_of(X, Z), knows(Z, Y).

% Who are Alice's friends-of-friends?
?- foaf(alice, Who).

% Who does Alice know (directly or indirectly)?
?- knows(alice, Who).

% Who does Bob know?
?- knows(bob, Who).

/* ==========================================
   FINAL: Summary
   ========================================== */

?- write(showcase_summary).

% Verify all Time Warp languages are represented
?- programming_language(X).

?- write(end_of_showcase).
