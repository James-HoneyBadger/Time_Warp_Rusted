{ ============================================ }
{ 06 - Procedures and Functions                 }
{ Learn: procedure, function, parameters        }
{ ============================================ }

program ProceduresAndFunctions;
var
  i, result: Integer;

procedure PrintLine(ch: string; count: Integer);
var
  i: Integer;
begin
  for i := 1 to count do
    write(ch);
  writeln;
end;

procedure PrintBanner(title: string);
begin
  PrintLine('=', 40);
  writeln('  ', title);
  PrintLine('=', 40);
end;

procedure PrintTriangle(rows: Integer);
var
  i, j: Integer;
begin
  for i := 1 to rows do
  begin
    for j := 1 to i do
      write('* ');
    writeln;
  end;
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
  i, found: Integer;
begin
  if n < 2 then
  begin
    IsPrime := 0;
  end
  else
  begin
    found := 1;
    for i := 2 to n - 1 do
      if n mod i = 0 then
        found := 0;
    IsPrime := found;
  end;
end;

function Power(base, exp: Integer): Integer;
var
  res, i: Integer;
begin
  res := 1;
  for i := 1 to exp do
    res := res * base;
  Power := res;
end;

begin
  PrintBanner('Procedures & Functions');
  writeln;

  { Triangle pattern }
  writeln('--- Star Triangle ---');
  PrintTriangle(5);
  writeln;

  { Factorials }
  writeln('--- Factorials ---');
  for i := 1 to 10 do
    writeln('  ', i, '! = ', Factorial(i));
  writeln;

  { Prime numbers }
  writeln('--- Primes up to 50 ---');
  write('  ');
  for i := 2 to 50 do
    if IsPrime(i) = 1 then
      write(i, ' ');
  writeln;
  writeln;

  { Powers }
  writeln('--- Powers of 2 ---');
  for i := 0 to 10 do
    writeln('  2^', i, ' = ', Power(2, i));
  writeln;

  PrintLine('-', 40);
  writeln('Done!');
end.
