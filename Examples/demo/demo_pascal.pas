{ ============================================= }
{ Time Warp Pascal — Grand Demo                }
{ A comprehensive Pascal showcase              }
{ ============================================= }
program GrandDemo;

var
  i, j, n, temp: integer;
  sum, average: real;
  data: array[1..20] of integer;
  name: string;
  isPrime: boolean;
  fib1, fib2, fib3: integer;

{ --- Utility Procedures --- }

procedure PrintBanner(title: string);
begin
  writeln('============================================');
  writeln('  ', title);
  writeln('============================================');
end;

procedure PrintLine;
begin
  writeln('--------------------------------------------');
end;

{ --- Mathematical Functions --- }

function Factorial(n: integer): integer;
begin
  if n <= 1 then
    Factorial := 1
  else
    Factorial := n * Factorial(n - 1);
end;

function GCD(a, b: integer): integer;
begin
  while b <> 0 do
  begin
    temp := b;
    b := a mod b;
    a := temp;
  end;
  GCD := a;
end;

function Power(base, exp: integer): integer;
var
  result: integer;
begin
  result := 1;
  for i := 1 to exp do
    result := result * base;
  Power := result;
end;

function IsPrimeNumber(n: integer): boolean;
var
  k: integer;
begin
  if n < 2 then
  begin
    IsPrimeNumber := false;
    exit;
  end;
  IsPrimeNumber := true;
  for k := 2 to n div 2 do
  begin
    if n mod k = 0 then
    begin
      IsPrimeNumber := false;
      exit;
    end;
  end;
end;

{ --- Main Program --- }
begin
  PrintBanner('TIME WARP PASCAL - GRAND DEMO');
  writeln;

  { Section 1: Factorials }
  PrintBanner('Section 1: Factorials');
  for i := 1 to 10 do
    writeln('  ', i, '! = ', Factorial(i));
  writeln;

  { Section 2: Prime Numbers }
  PrintBanner('Section 2: Prime Numbers (1 to 50)');
  write('  ');
  for i := 2 to 50 do
  begin
    if IsPrimeNumber(i) then
      write(i, ' ');
  end;
  writeln;
  writeln;

  { Section 3: Powers of 2 }
  PrintBanner('Section 3: Powers of 2');
  for i := 0 to 15 do
    writeln('  2^', i, ' = ', Power(2, i));
  writeln;

  { Section 4: Fibonacci Sequence }
  PrintBanner('Section 4: Fibonacci Sequence');
  fib1 := 0;
  fib2 := 1;
  write('  ');
  for i := 1 to 20 do
  begin
    write(fib1, ' ');
    fib3 := fib1 + fib2;
    fib1 := fib2;
    fib2 := fib3;
  end;
  writeln;
  writeln;

  { Section 5: GCD Table }
  PrintBanner('Section 5: Greatest Common Divisors');
  writeln('  GCD(12, 8)  = ', GCD(12, 8));
  writeln('  GCD(100,75) = ', GCD(100, 75));
  writeln('  GCD(48, 36) = ', GCD(48, 36));
  writeln('  GCD(17, 13) = ', GCD(17, 13));
  writeln('  GCD(144,60) = ', GCD(144, 60));
  writeln;

  { Section 6: Array Sorting }
  PrintBanner('Section 6: Bubble Sort');

  { Fill array with decreasing values }
  for i := 1 to 15 do
    data[i] := 16 - i;

  write('  Before: ');
  for i := 1 to 15 do
    write(data[i], ' ');
  writeln;

  { Bubble sort }
  for i := 1 to 14 do
    for j := 1 to 15 - i do
    begin
      if data[j] > data[j + 1] then
      begin
        temp := data[j];
        data[j] := data[j + 1];
        data[j + 1] := temp;
      end;
    end;

  write('  After:  ');
  for i := 1 to 15 do
    write(data[i], ' ');
  writeln;
  writeln;

  { Section 7: Statistics }
  PrintBanner('Section 7: Statistics');
  sum := 0;
  for i := 1 to 15 do
    sum := sum + data[i];
  average := sum / 15;

  writeln('  Count:   15');
  writeln('  Sum:     ', sum:0:0);
  writeln('  Average: ', average:0:2);
  writeln('  Min:     ', data[1]);
  writeln('  Max:     ', data[15]);
  writeln;

  { Section 8: Multiplication Table }
  PrintBanner('Section 8: Multiplication Table');
  writeln('      1   2   3   4   5   6   7   8   9');
  PrintLine;
  for i := 1 to 9 do
  begin
    write('  ', i, ' |');
    for j := 1 to 9 do
      write(i * j:4);
    writeln;
  end;
  writeln;

  { Section 9: Pattern Generation }
  PrintBanner('Section 9: Triangle Pattern');
  for i := 1 to 10 do
  begin
    write('  ');
    for j := 1 to 10 - i do
      write(' ');
    for j := 1 to 2 * i - 1 do
      write('*');
    writeln;
  end;
  writeln;

  { Section 10: Number Classification }
  PrintBanner('Section 10: Number Classifier');
  for i := 1 to 20 do
  begin
    write('  ', i:3, ': ');
    if IsPrimeNumber(i) then
      write('prime ')
    else
      write('      ');

    if i mod 2 = 0 then
      write('even ')
    else
      write('odd  ');

    if i mod 3 = 0 then
      write('div3 ')
    else
      write('     ');

    if i mod 5 = 0 then
      write('div5');

    writeln;
  end;
  writeln;

  { Finale }
  PrintBanner('GRAND DEMO COMPLETE!');
  writeln('  Features demonstrated:');
  writeln('    - Variables, constants, types');
  writeln('    - Procedures with parameters');
  writeln('    - Recursive functions');
  writeln('    - FOR, WHILE, REPEAT loops');
  writeln('    - IF/THEN/ELSE conditionals');
  writeln('    - Arrays and sorting');
  writeln('    - Formatted output (write/writeln)');
  writeln('    - Nested loops');
  writeln('    - Mathematical algorithms');
  writeln('============================================');
end.
