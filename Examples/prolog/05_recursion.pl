% =============================================
% Prolog Example 05 — Recursive Rules
% Time Warp Rusted
% =============================================
% Prolog rules can be recursive — a rule
% can refer to itself. This is how Prolog
% handles chains of relationships.

% --- Family tree ---
parent(alice, bob).
parent(alice, carol).
parent(bob, dave).
parent(bob, eve).
parent(carol, frank).
parent(dave, grace).
parent(dave, henry).
parent(frank, irene).

% --- Base case: X is an ancestor of Y if X is parent of Y ---
ancestor(X, Y) :- parent(X, Y).

% --- Recursive case: X is ancestor of Y if X is parent of Z
%     and Z is an ancestor of Y ---
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

% === Queries ===

% Direct ancestors of Grace
?- ancestor(X, grace).

% Is Alice an ancestor of Grace?
?- ancestor(alice, grace).

% Is Alice an ancestor of Irene?
?- ancestor(alice, irene).

% Who are all of Alice's descendants?
?- ancestor(alice, Descendant).

% Who are Bob's descendants?
?- ancestor(bob, Descendant).

% --- Another recursive example: connected graph ---
edge(a, b).
edge(b, c).
edge(c, d).
edge(d, e).
edge(a, f).
edge(f, g).

% Base: directly connected
connected(X, Y) :- edge(X, Y).

% Recursive: connected through intermediate nodes
connected(X, Y) :- edge(X, Z), connected(Z, Y).

% Is a connected to e?
?- connected(a, e).

% What is a connected to?
?- connected(a, Node).

% Is a connected to g?
?- connected(a, g).
