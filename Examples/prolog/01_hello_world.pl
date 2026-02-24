% =============================================
% Prolog Example 01 — Hello World & Basic Facts
% Time Warp Rusted
% =============================================
% Prolog programs consist of facts, rules,
% and queries. Facts state what is true.
% Queries ask questions with ?-

% --- Simple facts ---
greeting(hello).
greeting(hi).
greeting(welcome).

language(prolog).
language(basic).
language(logo).

% --- Query: check a fact ---
?- greeting(hello).

% --- Query: find all greetings ---
?- greeting(X).

% --- Query: check language ---
?- language(prolog).

% --- More facts ---
colour(red).
colour(green).
colour(blue).

?- colour(X).
