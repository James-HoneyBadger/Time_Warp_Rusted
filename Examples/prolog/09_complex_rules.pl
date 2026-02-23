% =============================================
% Prolog Example 09 — Complex Rules & Chains
% Time Warp Studio
% =============================================
% Demonstrates multi-goal rules, chained
% reasoning, and complex knowledge structures.

% --- Company hierarchy ---
manages(ceo, vp_eng).
manages(ceo, vp_sales).
manages(ceo, vp_hr).
manages(vp_eng, lead_backend).
manages(vp_eng, lead_frontend).
manages(vp_sales, sales_rep1).
manages(vp_sales, sales_rep2).
manages(lead_backend, dev1).
manages(lead_backend, dev2).
manages(lead_frontend, dev3).

% Direct report
reports_to(X, Y) :- manages(Y, X).

% Indirect chain of command
chain_of_command(X, Y) :- manages(Y, X).
chain_of_command(X, Y) :- manages(Z, X), chain_of_command(Z, Y).

% Same team (share a manager)
same_team(X, Y) :- manages(Z, X), manages(Z, Y).

% --- Queries ---

% Who does the CEO manage directly?
?- manages(ceo, X).

% Who does VP Engineering manage?
?- manages(vp_eng, X).

% Who reports to lead_backend?
?- reports_to(X, lead_backend).

% Dev1's full chain of command
?- chain_of_command(dev1, Boss).

% Who is on the same team as dev2?
?- same_team(dev2, Colleague).

% --- Movie database ---
movie(inception, nolan, 2010, scifi).
movie(interstellar, nolan, 2014, scifi).
movie(dark_knight, nolan, 2008, action).
movie(matrix, wachowski, 1999, scifi).
movie(avatar, cameron, 2009, scifi).
movie(titanic, cameron, 1997, drama).
movie(aliens, cameron, 1986, scifi).
movie(pulp_fiction, tarantino, 1994, crime).
movie(kill_bill, tarantino, 2003, action).

% Same director
same_director(X, Y) :- movie(X, D, _, _), movie(Y, D, _, _).

% Same genre
same_genre(X, Y) :- movie(X, _, _, G), movie(Y, _, _, G).

% --- Movie queries ---

% All Nolan films
?- movie(Title, nolan, _, _).

% All sci-fi films
?- movie(Title, _, _, scifi).

% Films by Cameron
?- movie(Title, cameron, Year, Genre).

% Movies by same director as Inception
?- same_director(inception, Other).

% Movies in same genre as Dark Knight
?- same_genre(dark_knight, Other).

% All movies
?- movie(Title, Director, Year, Genre).
