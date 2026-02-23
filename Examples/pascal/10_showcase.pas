{ ============================================ }
{ 10 - Complete Pascal Showcase                  }
{ Demonstrates EVERY Pascal feature             }
{ ============================================ }

program Showcase;
var
  i, j, n, sum: Integer;
  name: string;
const
  BANNER_WIDTH = 40;
  VERSION = 'Time Warp Studio';

procedure DrawLine(ch: string; width: Integer);
var
  i: Integer;
begin
  for i := 1 to width do
    write(ch);
  writeln;
end;

procedure DrawBox(text: string);
begin
  DrawLine('=', BANNER_WIDTH);
  writeln('  ', text);
  DrawLine('=', BANNER_WIDTH);
end;

function Factorial(n: Integer): Integer;
begin
  if n <= 1 then
    Factorial := 1
  else
    Factorial := n * Factorial(n - 1);
end;

function IsPrime(n: Integer): Integer;
var
  i: Integer;
begin
  if n < 2 then
  begin
    IsPrime := 0;
  end
  else
  begin
    IsPrime := 1;
    for i := 2 to n - 1 do
      if n mod i = 0 then
        IsPrime := 0;
  end;
end;

function Power(base, exp: Integer): Integer;
var
  result, i: Integer;
begin
  result := 1;
  for i := 1 to exp do
    result := result * base;
  Power := result;
end;

begin
  DrawBox(VERSION);
  writeln;

  { --- Variables & Constants --- }
  writeln('--- Variables & Constants ---');
  name := 'Pascal Programmer';
  n := 42;
  writeln('Name: ', name);
  writeln('N: ', n);
  writeln('PI: ', 3.14159);
  writeln;

  { --- Arithmetic --- }
  writeln('--- Arithmetic ---');
  writeln('25 + 7 = ', 25 + 7);
  writeln('25 * 7 = ', 25 * 7);
  writeln('100 / 3 = ', 100 / 3);
  writeln('100 mod 3 = ', 100 mod 3);
  writeln;

  { --- Conditionals --- }
  writeln('--- Conditionals ---');
  n := 85;
  if n >= 90 then
    writeln('Grade A')
  else if n >= 80 then
    writeln('Score ', n, ': Grade B')
  else
    writeln('Below B');
  writeln;

  { --- FOR Loop --- }
  writeln('--- FOR Loop (squares) ---');
  for i := 1 to 8 do
    write(i * i, ' ');
  writeln;

  writeln('--- FOR DOWNTO ---');
  for i := 5 downto 1 do
    write(i, ' ');
  writeln;
  writeln;

  { --- WHILE Loop --- }
  writeln('--- WHILE Loop (Fibonacci) ---');
  i := 0;
  j := 1;
  while i < 100 do
  begin
    write(i, ' ');
    n := i + j;
    i := j;
    j := n;
  end;
  writeln;
  writeln;

  { --- REPEAT/UNTIL --- }
  writeln('--- REPEAT/UNTIL ---');
  n := 1;
  i := 0;
  repeat
    write(n, ' ');
    n := n * 2;
    i := i + 1;
  until n > 100;
  writeln;
  writeln;

  { --- Procedures & Functions --- }
  writeln('--- Functions ---');
  for i := 1 to 8 do
    writeln('  ', i, '! = ', Factorial(i));
  writeln;

  writeln('--- Primes to 50 ---');
  write('  ');
  for i := 2 to 50 do
    if IsPrime(i) = 1 then
      write(i, ' ');
  writeln;
  writeln;

  { --- Nested Loops --- }
  writeln('--- Pattern ---');
  for i := 1 to 5 do
  begin
    for j := 1 to i do
      write('* ');
    writeln;
  end;
  for i := 4 downto 1 do
  begin
    for j := 1 to i do
      write('* ');
    writeln;
  end;
  writeln;

  { --- Sum --- }
  sum := 0;
  for i := 1 to 100 do
    sum := sum + i;
  writeln('Sum(1..100) = ', sum);
  writeln;

  DrawBox('All Pascal features demonstrated!');
end.
