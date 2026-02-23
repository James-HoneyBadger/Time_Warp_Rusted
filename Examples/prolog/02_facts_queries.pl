% =============================================
% Prolog Example 02 — Facts & Queries
% Time Warp Studio
% =============================================
% Facts describe relationships between things.
% Queries with variables (uppercase) find
% matching values through unification.

% --- Family facts ---
parent(tom, bob).
parent(tom, liz).
parent(bob, ann).
parent(bob, pat).
parent(pat, jim).
parent(liz, mia).

% --- Gender facts ---
male(tom).
male(bob).
male(pat).
male(jim).
female(liz).
female(ann).
female(mia).

% === Queries ===

% Who are Tom's children?
?- parent(tom, X).

% Who is Bob's parent?
?- parent(X, bob).

% Who are all the parents?
?- parent(X, _).

% Who is male?
?- male(X).

% Who is female?
?- female(X).

% Is Tom the parent of Bob?
?- parent(tom, bob).

% Is Tom the parent of Ann?
?- parent(tom, ann).

% Who are Bob's children?
?- parent(bob, Child).

% Find parent-child pairs
?- parent(Parent, Child).
