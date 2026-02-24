% =============================================
% Prolog Example 08 — Write & Output
% Time Warp Rusted
% =============================================
% The write() predicate outputs a value.
% Useful for displaying results and messages.

% --- Simple output ---
?- write(hello).
?- write(world).

% --- Output facts ---
greeting(hello_world).
greeting(good_morning).
greeting(welcome).

% We can query and the system shows bindings
?- greeting(X).

% --- A small database to display ---
student(alice, math, a).
student(bob, math, b).
student(carol, science, a).
student(dave, science, c).
student(eve, english, a).
student(frank, english, b).

% Find all students
?- student(Name, Subject, Grade).

% Find all A students
?- student(Name, _, a).

% Find math students
?- student(Name, math, Grade).

% --- Using write for custom output ---
?- write(results_displayed).

% --- Capital cities ---
capital(france, paris).
capital(germany, berlin).
capital(japan, tokyo).
capital(australia, canberra).
capital(brazil, brasilia).
capital(canada, ottawa).

% What is the capital of France?
?- capital(france, City).

% Which country has Tokyo as capital?
?- capital(Country, tokyo).

% All capitals
?- capital(Country, City).
