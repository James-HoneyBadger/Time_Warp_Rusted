% =============================================
% Prolog Example 04 — Unification
% Time Warp Studio
% =============================================
% Unification is at the heart of Prolog.
% When a query is made, Prolog tries to
% match (unify) variables with values.
% The anonymous variable _ matches anything.

% --- Facts about animals ---
animal(cat, mammal, domestic).
animal(dog, mammal, domestic).
animal(eagle, bird, wild).
animal(parrot, bird, domestic).
animal(shark, fish, wild).
animal(goldfish, fish, domestic).
animal(frog, amphibian, wild).
animal(python, reptile, wild).

% --- Query with specific values ---
% Is there a domestic mammal?
?- animal(X, mammal, domestic).

% --- Query with anonymous variable ---
% Find all bird names (ignore other fields)
?- animal(X, bird, _).

% --- Find all domestic animals ---
?- animal(Name, _, domestic).

% --- Find all wild animals ---
?- animal(Name, _, wild).

% --- Find all mammals ---
?- animal(Name, mammal, _).

% --- Find all fish ---
?- animal(Name, fish, _).

% --- Specific checks ---
% Is there a domestic bird?
?- animal(X, bird, domestic).

% Is there a wild mammal?
?- animal(X, mammal, wild).

% --- Multi-argument facts ---
book(hobbit, tolkien, 1937).
book(dune, herbert, 1965).
book(foundation, asimov, 1951).
book(neuromancer, gibson, 1984).
book(snow_crash, stephenson, 1992).

% Find all books by their author
?- book(Title, Author, _).

% Find books published before 1960
% (Note: Prolog in Time Warp uses unification,
%  not arithmetic comparison in queries)
?- book(Title, _, Year).
