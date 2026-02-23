{ ============================================ }
{ 03 - Input and Output                         }
{ Learn: readln, write, writeln, formatting     }
{ ============================================ }

program InputOutput;
var
  name: string;
  age: Integer;
  num1, num2: Integer;
begin
  writeln('=== Interactive I/O ===');
  writeln;

  { String input }
  write('What is your name? ');
  readln(name);
  writeln('Hello, ', name, '!');
  writeln;

  { Numeric input }
  write('How old are you? ');
  readln(age);
  writeln('You will be ', age + 10, ' in ten years.');
  writeln;

  { Calculator }
  writeln('=== Mini Calculator ===');
  write('First number: ');
  readln(num1);
  write('Second number: ');
  readln(num2);
  writeln;
  writeln('Results:');
  writeln('  ', num1, ' + ', num2, ' = ', num1 + num2);
  writeln('  ', num1, ' - ', num2, ' = ', num1 - num2);
  writeln('  ', num1, ' * ', num2, ' = ', num1 * num2);
  if num2 <> 0 then
    writeln('  ', num1, ' / ', num2, ' = ', num1 / num2)
  else
    writeln('  Cannot divide by zero!');
  writeln;
  writeln('Thank you, ', name, '!');
end.
