% =============================================
% Prolog Example 03 — Rules
% Time Warp Studio
% =============================================
% Rules define new relationships based on
% existing facts. Format: head :- body.
% "head is true if body is true"

% --- Facts ---
parent(tom, bob).
parent(tom, liz).
parent(bob, ann).
parent(bob, pat).
parent(pat, jim).

male(tom).
male(bob).
male(pat).
male(jim).
female(liz).
female(ann).

% --- Rules ---

% X is the father of Y if X is parent of Y and X is male
father(X, Y) :- parent(X, Y), male(X).

% X is the mother of Y if X is parent of Y and X is female
mother(X, Y) :- parent(X, Y), female(X).

% X is a sibling of Y if they share a parent (and are different)
sibling(X, Y) :- parent(Z, X), parent(Z, Y).

% X is a grandparent of Y if X is parent of Z and Z is parent of Y
grandparent(X, Y) :- parent(X, Z), parent(Z, Y).

% === Queries ===

% Who is a father?
?- father(X, Y).

% Who is Tom the father of?
?- father(tom, Child).

% Who are Bob's siblings?
?- sibling(X, bob).

% Who are the grandparents?
?- grandparent(X, Y).

% Is Tom a grandparent of Ann?
?- grandparent(tom, ann).

% Is Tom a grandparent of Jim?
?- grandparent(tom, jim).
