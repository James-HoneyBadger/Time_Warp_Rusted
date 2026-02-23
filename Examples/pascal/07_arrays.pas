{ ============================================ }
{ 07 - Arrays and Algorithms                    }
{ Learn: arrays, sorting, searching             }
{ ============================================ }

program ArraysAndAlgorithms;
var
  data: array[1..10] of Integer;
  sorted: array[1..10] of Integer;
  i, j, temp, sum, min_val, max_val: Integer;
begin
  writeln('=== Arrays & Algorithms ===');
  writeln;

  { Initialize array }
  data[1] := 64;  data[2] := 34;  data[3] := 25;
  data[4] := 12;  data[5] := 22;  data[6] := 11;
  data[7] := 90;  data[8] := 45;  data[9] := 78;
  data[10] := 56;

  { Display original }
  writeln('Original array:');
  write('  ');
  for i := 1 to 10 do
    write(data[i], ' ');
  writeln;
  writeln;

  { Statistics }
  sum := 0;
  min_val := data[1];
  max_val := data[1];
  for i := 1 to 10 do
  begin
    sum := sum + data[i];
    if data[i] < min_val then
      min_val := data[i];
    if data[i] > max_val then
      max_val := data[i];
  end;
  writeln('Statistics:');
  writeln('  Sum:     ', sum);
  writeln('  Average: ', sum / 10);
  writeln('  Minimum: ', min_val);
  writeln('  Maximum: ', max_val);
  writeln;

  { Copy to sorted array }
  for i := 1 to 10 do
    sorted[i] := data[i];

  { Bubble sort }
  for i := 1 to 9 do
    for j := 1 to 10 - i do
      if sorted[j] > sorted[j + 1] then
      begin
        temp := sorted[j];
        sorted[j] := sorted[j + 1];
        sorted[j + 1] := temp;
      end;

  writeln('Sorted array:');
  write('  ');
  for i := 1 to 10 do
    write(sorted[i], ' ');
  writeln;
  writeln;

  { Linear search }
  writeln('Searching for 45...');
  for i := 1 to 10 do
    if data[i] = 45 then
      writeln('  Found at position ', i);

  writeln;
  writeln('Done!');
end.
