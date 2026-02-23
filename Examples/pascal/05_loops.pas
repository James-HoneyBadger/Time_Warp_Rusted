{ ============================================ }
{ 05 - Loops                                    }
{ Learn: for, while, repeat/until, nested       }
{ ============================================ }

program Loops;
var
  i, j, sum, n: Integer;
  a, b, temp: Integer;
begin
  writeln('=== Loop Structures ===');
  writeln;

  { FOR loop counting up }
  writeln('--- FOR loop (1 to 10) ---');
  for i := 1 to 10 do
    write(i, ' ');
  writeln;
  writeln;

  { FOR loop counting down }
  writeln('--- FOR DOWNTO ---');
  for i := 10 downto 1 do
    write(i, ' ');
  writeln('Liftoff!');
  writeln;

  { Nested FOR - multiplication table }
  writeln('--- Multiplication Table ---');
  writeln('    1   2   3   4   5');
  writeln('  --------------------');
  for i := 1 to 5 do
  begin
    write(i, ' | ');
    for j := 1 to 5 do
      write(i * j, '  ');
    writeln;
  end;
  writeln;

  { WHILE loop }
  writeln('--- WHILE loop (Fibonacci) ---');
  a := 0;
  b := 1;
  while a < 100 do
  begin
    write(a, ' ');
    temp := a + b;
    a := b;
    b := temp;
  end;
  writeln;
  writeln;

  { REPEAT/UNTIL }
  writeln('--- REPEAT/UNTIL (Powers of 2) ---');
  n := 1;
  i := 0;
  repeat
    writeln('  2^', i, ' = ', n);
    n := n * 2;
    i := i + 1;
  until n > 1024;
  writeln;

  { Sum calculation }
  sum := 0;
  for i := 1 to 100 do
    sum := sum + i;
  writeln('Sum of 1 to 100 = ', sum);
end.
