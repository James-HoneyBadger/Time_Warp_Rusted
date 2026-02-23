{ ============================================ }
{ 02 - Variables and Types                      }
{ Learn: var, const, integer, string, assign    }
{ ============================================ }

program VariablesAndTypes;
var
  age: Integer;
  height: Integer;
  name: string;
  greeting: string;
const
  PI = 3.14159;
  GRAVITY = 9.81;
  SCHOOL = 'Time Warp Academy';
begin
  writeln('=== Variables & Types ===');
  writeln;

  { Integer variables }
  age := 15;
  height := 165;
  writeln('Age: ', age);
  writeln('Height: ', height, ' cm');
  writeln;

  { String variables }
  name := 'Pascal Student';
  greeting := 'Hello, ';
  writeln(greeting, name, '!');
  writeln;

  { Constants }
  writeln('=== Constants ===');
  writeln('PI = ', PI);
  writeln('Gravity = ', GRAVITY, ' m/s^2');
  writeln('School: ', SCHOOL);
  writeln;

  { Arithmetic }
  writeln('=== Math ===');
  writeln('25 + 7 = ', 25 + 7);
  writeln('25 - 7 = ', 25 - 7);
  writeln('25 * 7 = ', 25 * 7);
  writeln('25 / 7 = ', 25 / 7);
  writeln('Circle area (r=10): ', PI * 10 * 10);
end.
