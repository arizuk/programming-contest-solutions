N = STDIN.gets.to_i
aa = STDIN.gets.strip.split.map(&:to_i)

multiples = {
  0 => 0,
  2 => 0,
  4 => 0,
}

aa.each do |a|
  if a % 4 == 0
    multiples[4] += 1
  elsif a % 2 == 0
    multiples[2] += 1
  else
    multiples[0] += 1
  end
end

if multiples[2] == 0
  puts multiples[0]-1 <= multiples[4] ? "Yes" : "No"
else
  puts multiples[0] <= multiples[4] ? "Yes" : "No"
end