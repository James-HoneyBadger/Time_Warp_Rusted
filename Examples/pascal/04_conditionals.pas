{ ============================================ }
{ 04 - Conditionals                             }
{ Learn: if/then/else, nested conditions        }
{ ============================================ }

program Conditionals;
var
  score, temp, hour: Integer;
begin
  writeln('=== Conditionals ===');
  writeln;

  { Simple if/then }
  score := 85;
  writeln('Score: ', score);
  if score >= 90 then
    writeln('Grade: A - Excellent!')
  else if score >= 80 then
    writeln('Grade: B - Great work!')
  else if score >= 70 then
    writeln('Grade: C - Good job')
  else if score >= 60 then
    writeln('Grade: D - Needs improvement')
  else
    writeln('Grade: F - See teacher');
  writeln;

  { Nested conditions with begin/end }
  temp := 72;
  writeln('Temperature: ', temp, ' degrees');
  if temp > 85 then
  begin
    writeln('  It is HOT outside!');
    writeln('  Stay hydrated!');
  end
  else if temp > 60 then
  begin
    writeln('  Nice weather!');
    writeln('  Perfect for a walk.');
  end
  else
  begin
    writeln('  It is cold!');
    writeln('  Wear a jacket.');
  end;
  writeln;

  { Time-based greeting }
  hour := 14;
  writeln('Hour: ', hour, ':00');
  if hour < 12 then
    writeln('Good morning!')
  else if hour < 17 then
    writeln('Good afternoon!')
  else
    writeln('Good evening!');
  writeln;

  { Boolean expressions }
  if (score > 80) and (temp > 60) then
    writeln('Great score AND nice weather!');
  if (score > 90) or (temp > 90) then
    writeln('Either excellent score or very hot')
  else
    writeln('Neither top score nor extreme heat');
end.
