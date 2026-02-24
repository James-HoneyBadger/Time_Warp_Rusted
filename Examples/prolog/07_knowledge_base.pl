% =============================================
% Prolog Example 07 — Knowledge Base
% Time Warp Rusted
% =============================================
% Prolog excels at representing knowledge.
% This example builds a small expert system
% about the solar system.

% --- Planet facts ---
% planet(Name, Type, Position, Moons)
planet(mercury, rocky, 1, 0).
planet(venus, rocky, 2, 0).
planet(earth, rocky, 3, 1).
planet(mars, rocky, 4, 2).
planet(jupiter, gas_giant, 5, 95).
planet(saturn, gas_giant, 6, 146).
planet(uranus, ice_giant, 7, 27).
planet(neptune, ice_giant, 8, 16).

% --- Rules ---
% A planet is inner if position <= 4
inner_planet(X) :- planet(X, rocky, _, _).

% A planet is outer if it is a gas or ice giant
outer_planet(X) :- planet(X, gas_giant, _, _).
outer_planet(X) :- planet(X, ice_giant, _, _).

% A planet has moons
has_moons(X) :- planet(X, _, _, M), M > 0.

% actually we can't do arithmetic comparison easily,
% let's use a different approach
% A planet with no moons
moonless(mercury).
moonless(venus).

% Ringed planets
ringed(saturn).
ringed(jupiter).
ringed(uranus).
ringed(neptune).

% --- Queries ---

% All planets
?- planet(Name, _, _, _).

% All rocky planets
?- planet(Name, rocky, _, _).

% All gas giants
?- planet(Name, gas_giant, _, _).

% All ice giants
?- planet(Name, ice_giant, _, _).

% Inner planets
?- inner_planet(X).

% Outer planets
?- outer_planet(X).

% Moonless planets
?- moonless(X).

% Ringed planets
?- ringed(X).

% --- Element knowledge base ---
element(hydrogen, nonmetal, 1).
element(helium, noble_gas, 2).
element(lithium, metal, 3).
element(carbon, nonmetal, 6).
element(nitrogen, nonmetal, 7).
element(oxygen, nonmetal, 8).
element(iron, metal, 26).
element(gold, metal, 79).
element(silver, metal, 47).

% All metals
?- element(Name, metal, _).

% All nonmetals
?- element(Name, nonmetal, _).

% All elements
?- element(Name, Type, Number).
