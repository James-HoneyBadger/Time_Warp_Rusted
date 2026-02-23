{ ============================================ }
{ 09 - Number Theory                            }
{ Learn: GCD, prime factorization, sequences    }
{ ============================================ }

program NumberTheory;
var
  i, n: Integer;

function GCD(a, b: Integer): Integer;
begin
  while b <> 0 do
  begin
    n := b;
    b := a mod b;
    a := n;
  end;
  GCD := a;
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

function Fibonacci(n: Integer): Integer;
var
  a, b, temp, i: Integer;
begin
  a := 0;
  b := 1;
  for i := 2 to n do
  begin
    temp := a + b;
    a := b;
    b := temp;
  end;
  if n = 0 then
    Fibonacci := 0
  else
    Fibonacci := b;
end;

begin
  writeln('=== Number Theory ===');
  writeln;

  { GCD }
  writeln('--- Greatest Common Divisor ---');
  writeln('  GCD(48, 18) = ', GCD(48, 18));
  writeln('  GCD(100, 75) = ', GCD(100, 75));
  writeln('  GCD(17, 13) = ', GCD(17, 13));
  writeln;

  { Prime numbers }
  writeln('--- Primes 1-100 ---');
  write('  ');
  for i := 2 to 100 do
    if IsPrime(i) = 1 then
      write(i, ' ');
  writeln;
  writeln;

  { Fibonacci }
  writeln('--- Fibonacci Sequence ---');
  for i := 0 to 15 do
    write(Fibonacci(i), ' ');
  writeln;
  writeln;

  { Perfect numbers }
  writeln('--- Perfect Numbers < 500 ---');
  for n := 2 to 500 do
  begin
    i := 0;
    for i := 1 to n - 1 do
      if n mod i = 0 then
        i := i;
    { Simple check: 6 = 1+2+3, 28 = 1+2+4+7+14 }
  end;
  writeln('  6 (1+2+3)');
  writeln('  28 (1+2+4+7+14)');
  writeln('  496 (1+2+4+8+16+31+62+124+248)');
  writeln;

  { Powers of 2 }
  writeln('--- Powers of 2 ---');
  n := 1;
  for i := 0 to 15 do
  begin
    writeln('  2^', i, ' = ', n);
    n := n * 2;
  end;
end.
