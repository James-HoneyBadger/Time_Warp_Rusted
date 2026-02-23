{ ============================================ }
{ 08 - Interactive Calculator                   }
{ Learn: Combining input, math, procedures      }
{ ============================================ }

program Calculator;
var
  a, b: Integer;
  choice: Integer;

procedure ShowMenu;
begin
  writeln;
  writeln('=== Pascal Calculator ===');
  writeln('  1. Addition');
  writeln('  2. Subtraction');
  writeln('  3. Multiplication');
  writeln('  4. Division');
  writeln('  5. Power');
  writeln('  6. Factorial');
  writeln('  0. Exit');
  writeln;
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

function Factorial(n: Integer): Integer;
begin
  if n <= 1 then
    Factorial := 1
  else
    Factorial := n * Factorial(n - 1);
end;

begin
  ShowMenu;

  write('Choose operation (0-6): ');
  readln(choice);

  if choice = 0 then
    writeln('Goodbye!')
  else if choice = 6 then
  begin
    write('Enter number: ');
    readln(a);
    writeln(a, '! = ', Factorial(a));
  end
  else
  begin
    write('Enter first number: ');
    readln(a);
    write('Enter second number: ');
    readln(b);
    writeln;

    if choice = 1 then
      writeln(a, ' + ', b, ' = ', a + b)
    else if choice = 2 then
      writeln(a, ' - ', b, ' = ', a - b)
    else if choice = 3 then
      writeln(a, ' * ', b, ' = ', a * b)
    else if choice = 4 then
    begin
      if b <> 0 then
        writeln(a, ' / ', b, ' = ', a / b)
      else
        writeln('Error: Division by zero!');
    end
    else if choice = 5 then
      writeln(a, ' ^ ', b, ' = ', Power(a, b))
    else
      writeln('Invalid choice!');
  end;
end.
