% =============================================
% Prolog Example 06 — Backtracking
% Time Warp Rusted
% =============================================
% Prolog automatically backtracks to find
% ALL possible solutions. When one path fails,
% it tries the next possibility.

% --- A maze with multiple paths ---
door(entrance, hallway).
door(hallway, kitchen).
door(hallway, library).
door(kitchen, garden).
door(library, study).
door(study, garden).
door(garden, exit).

% Can navigate through doors
can_reach(X, Y) :- door(X, Y).
can_reach(X, Y) :- door(X, Z), can_reach(Z, Y).

% Find all rooms reachable from entrance
?- can_reach(entrance, Room).

% Can we get from entrance to exit?
?- can_reach(entrance, exit).

% Can we reach the garden from the hallway?
?- can_reach(hallway, garden).

% --- Multiple solutions for food preferences ---
likes(alice, pizza).
likes(alice, pasta).
likes(alice, salad).
likes(bob, pizza).
likes(bob, burger).
likes(carol, pasta).
likes(carol, salad).
likes(carol, sushi).

% What does Alice like?
?- likes(alice, What).

% Who likes pizza?
?- likes(Who, pizza).

% What do Alice and Carol both like?
% (Find X where both like it)
both_like(X) :- likes(alice, X), likes(carol, X).

?- both_like(Food).

% --- Course prerequisites ---
prereq(math101, math201).
prereq(math201, math301).
prereq(cs101, cs201).
prereq(cs201, cs301).
prereq(math201, cs201).

% Direct or indirect prerequisite
requires(X, Y) :- prereq(X, Y).
requires(X, Y) :- prereq(X, Z), requires(Z, Y).

% What does CS301 require (directly or indirectly)?
?- requires(X, cs301).

% What courses require Math101 as a foundation?
?- requires(math101, Course).
