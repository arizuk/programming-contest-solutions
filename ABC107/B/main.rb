h, w = STDIN.gets.split(" ").map(&:to_i)
map = []
h.times do
  map << STDIN.gets.strip.split("")
end

for i in 0...h do
  if map[i].all? { |v| v == '.' }
    for j in 0...w do
      map[i][j] = nil
    end
  end
end

for j in 0...w do
  white = true
  for i in 0...h do
    unless map[i][j] == '.' || map[i][j] == nil
      white = false
    end
  end

  if white
    for i in 0...h do
      map[i][j] = nil
    end
  end
end

for i in 0...h do
  row = map[i].compact
  puts row.join('') if row.size > 0
end
