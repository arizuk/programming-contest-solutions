G = [
  "W........WW.",
  ".WWW.....WWW",
  "....WW...WW.",
  ".........WW.",
  ".........W..",
  "..W......W..",
  ".W.W.....WW.",
  "W.W.W.....W.",
  ".W.W......W.",
  "..W.......W.",
]
for i in 0...G.length do
  G[i] = G[i].split(//).map { |s| s == 'W' ? 1 : 0 }
end

N = G.length
M = G[0].length
MK = Array.new(N).map { Array.new(M, nil) }

def mark(i, j, w)
  return if MK[i][j] != nil
  if G[i][j] == 1
    MK[i][j] = w
    [-1, 0, 1].each do |dx|
      [-1, 0, 1].each do |dy|
        next if dx == 0 && dy == 0
        x = i + dx
        y = j + dy
        next if x < 0 || x >= N
        next if y < 0 || y >= M
        mark(x, y, w)
      end
    end
    true
  else
    MK[i][j] = 0
    false
  end
end

def print_w(a)
  a.each do |r|
    puts r.map { |v| v == 0 ? '-' : v }.join('')
  end
end

w = 1
for i in 0...N do
  for j in 0...M do
    if mark(i, j, w)
      w += 1
    end
  end
end
print_w(MK)
puts w - 1