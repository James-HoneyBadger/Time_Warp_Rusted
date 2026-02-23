# Tutorial 08 — Prolog: Logic Programming

## Introduction

Prolog (Programming in Logic) was created by Alain Colmerauer in 1972.
Unlike other languages where you tell the computer **how** to do something,
in Prolog you tell it **what** is true, and it figures out the answers itself
through logical reasoning.

**What you'll learn:**
- Facts (stating what is true)
- Queries (asking questions)
- Variables and unification
- Rules (defining new relationships)
- Recursion
- Backtracking (finding multiple answers)
- Knowledge bases

---

## Lesson 1: Facts

Facts state simple truths about the world:

```prolog
% These are facts
cat(tom).
cat(whiskers).
dog(rex).
dog(buddy).
```

A fact has a **functor** (name) and **arguments** in parentheses,
ending with a period `.`

Think of facts as entries in a database:
- "Tom is a cat"
- "Whiskers is a cat"
- "Rex is a dog"
- "Buddy is a dog"

### Multi-argument Facts

```prolog
% person(Name, Age)
person(alice, 14).
person(bob, 16).
person(carol, 13).

% likes(Who, What)
likes(alice, pizza).
likes(alice, coding).
likes(bob, football).
likes(bob, pizza).
```

**Challenge:** Create facts about 5 countries: their capital city and continent.

---

## Lesson 2: Queries

Queries ask Prolog questions. They start with `?-`:

```prolog
cat(tom).
cat(whiskers).
dog(rex).

% Ask: Is Tom a cat?
?- cat(tom).
% Output: true.

% Ask: Is Rex a cat?
?- cat(rex).
% Output: false.
```

### Queries with Variables

Use **uppercase** names for variables — Prolog will find matching values:

```prolog
cat(tom).
cat(whiskers).
dog(rex).

% Ask: What is a cat?
?- cat(X).
% Output: X = tom
%         X = whiskers
%         (2 solutions)

% Ask: What is a dog?
?- dog(X).
% Output: X = rex
```

**Key insight:** Prolog automatically finds ALL answers by trying every fact!

**Challenge:** Create animal facts and query: "What animals exist?"

---

## Lesson 3: The Anonymous Variable

Use `_` (underscore) when you don't care about a value:

```prolog
person(alice, 14, london).
person(bob, 16, paris).
person(carol, 13, london).
person(dave, 15, tokyo).

% Find all names (ignore age and city)
?- person(Name, _, _).

% Find everyone in London (ignore age)
?- person(Name, _, london).

% Find everyone's age (ignore name and city)
?- person(_, Age, _).
```

**Challenge:** Create a database of books (title, author, year) and use `_`
to query just titles, or just authors.

---

## Lesson 4: Rules

Rules define new relationships based on existing facts:

```prolog
% Facts
parent(tom, bob).
parent(tom, liz).
parent(bob, ann).
parent(bob, pat).

male(tom).
male(bob).
female(liz).
female(ann).

% Rules
father(X, Y) :- parent(X, Y), male(X).
mother(X, Y) :- parent(X, Y), female(X).
sibling(X, Y) :- parent(Z, X), parent(Z, Y).
```

Read `:-` as "**if**" and `,` as "**and**":
- "X is the father of Y **if** X is a parent of Y **and** X is male"
- "X and Y are siblings **if** Z is parent of X **and** Z is parent of Y"

```prolog
% Ask: Who is a father?
?- father(X, Y).
% X = tom, Y = bob
% X = tom, Y = liz
% X = bob, Y = ann
% etc.
```

**Challenge:** Add a `grandparent` rule and query for all grandparent-grandchild pairs.

---

## Lesson 5: Recursion

Rules can refer to themselves, creating chains of reasoning:

```prolog
parent(alice, bob).
parent(bob, carol).
parent(carol, dave).
parent(dave, eve).

% Base case: X is ancestor of Y if X is parent of Y
ancestor(X, Y) :- parent(X, Y).

% Recursive case: X is ancestor of Y if X is parent of Z
%                 and Z is ancestor of Y
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).
```

```prolog
% Is Alice an ancestor of Eve?
?- ancestor(alice, eve).
% true. (alice → bob → carol → dave → eve)

% Who are all of Alice's descendants?
?- ancestor(alice, Descendant).
% Descendant = bob
% Descendant = carol
% Descendant = dave
% Descendant = eve
```

**How it works:**
1. Is Alice a parent of Eve? No.
2. Is Alice a parent of someone who is an ancestor of Eve?
   - Alice is parent of Bob. Is Bob an ancestor of Eve?
     - Bob is parent of Carol. Is Carol an ancestor of Eve?
       - Carol is parent of Dave. Is Dave an ancestor of Eve?
         - Dave is parent of Eve. YES!

Prolog traces through the chain automatically!

**Challenge:** Create a "connected cities" database and a recursive
`can_reach(X, Y)` rule. Query which cities can reach which.

---

## Lesson 6: Backtracking

Prolog doesn't stop at the first answer — it backtracks to find ALL solutions:

```prolog
likes(alice, pizza).
likes(alice, pasta).
likes(alice, salad).
likes(bob, pizza).
likes(bob, burgers).
likes(carol, pasta).
likes(carol, salad).

% What does Alice like?
?- likes(alice, Food).
% Food = pizza
% Food = pasta
% Food = salad
% (3 solutions)

% Who likes pizza?
?- likes(Person, pizza).
% Person = alice
% Person = bob
% (2 solutions)
```

### Finding Shared Interests

```prolog
% Rule: both_like(Food) if alice and carol both like it
both_like(Food) :- likes(alice, Food), likes(carol, Food).

?- both_like(X).
% X = pasta
% X = salad
```

Prolog tries every combination to find all matches!

**Challenge:** Create a database of students and the subjects they take.
Write a rule to find students who share a subject.

---

## Lesson 7: Knowledge Bases

Prolog is perfect for building structured knowledge:

```prolog
% Animal classification
animal(dog, mammal, 4, domestic).
animal(cat, mammal, 4, domestic).
animal(eagle, bird, 2, wild).
animal(salmon, fish, 0, wild).
animal(frog, amphibian, 4, wild).

% Classification rules
is_pet(X) :- animal(X, _, _, domestic).
has_legs(X) :- animal(X, _, Legs, _).
is_wild(X) :- animal(X, _, _, wild).

% Queries
?- is_pet(X).
?- animal(X, mammal, _, _).
?- animal(X, bird, _, _).
```

### A Bigger Example

```prolog
% Elements
element(hydrogen, 1, nonmetal).
element(helium, 2, noble_gas).
element(carbon, 6, nonmetal).
element(oxygen, 8, nonmetal).
element(iron, 26, metal).
element(gold, 79, metal).
element(silver, 47, metal).

% Queries
?- element(Name, _, metal).      % All metals
?- element(Name, _, nonmetal).   % All nonmetals
?- element(Name, Number, Type).  % Everything
```

**Challenge:** Build a knowledge base about countries (name, continent,
language, population size). Write rules to find all European countries,
all English-speaking countries, etc.

---

## Lesson 8: The write() Predicate

Use `write()` to output values:

```prolog
?- write(hello).
?- write(time_warp).
?- write(prolog_is_fun).
```

`write()` is useful for displaying messages within your programs.

---

## Lesson 9: Multi-line Clauses

Long rules can span multiple lines — Prolog collects them until it sees a `.`

```prolog
% This rule spans multiple lines
qualified(Person) :-
    person(Person, Age, _),
    has_skill(Person, programming),
    Age > 18.
```

The clause continues until the period `.` at the end.

---

## How Prolog Thinks

Understanding Prolog's reasoning process:

1. **Receive a query** — e.g., `?- ancestor(alice, eve).`
2. **Try to match** against facts first
3. **Try rules** — substitute variables and try to prove the body
4. **Backtrack** if a path fails — try the next possibility
5. **Report solutions** — all variable bindings that make the query true

This is fundamentally different from other languages: you describe
**what** you want, not **how** to compute it. Prolog figures out the "how"!

---

## Projects

### Project 1: Family Tree
Create a complete family tree (at least 3 generations) with facts for
parent, male, female. Write rules for: father, mother, grandparent,
sibling, uncle, aunt, cousin. Query each relationship.

### Project 2: Animal Classifier
Build a knowledge base of 20+ animals with classifications. Write rules
to answer questions like: "What domestic mammals exist?" or
"Which animals have wings?"

### Project 3: Geography Database
Facts about countries, cities, rivers, mountains. Rules to find
neighbouring countries, countries on the same continent, etc.

### Project 4: Course Prerequisite Checker
Model university courses and their prerequisites. Write a recursive
rule to find all prerequisites (direct and indirect) for any course.

---

## Quick Reference

| Feature | Syntax | Example |
|---------|--------|---------|
| Fact | `functor(args).` | `cat(tom).` |
| Rule | `head :- body.` | `father(X,Y) :- parent(X,Y), male(X).` |
| Query | `?- goal.` | `?- cat(X).` |
| Variable | Uppercase | `X`, `Name`, `Who` |
| Anonymous | `_` | `person(Name, _, _)` |
| AND | `,` | `parent(X,Y), male(X)` |
| Write | `write(term)` | `?- write(hello).` |
| Comment | `%` | `% This is a comment` |
| Block comment | `/* ... */` | `/* Comment */` |
| Recursion | Rule calls itself | `ancestor(X,Y) :- parent(X,Z), ancestor(Z,Y).` |
