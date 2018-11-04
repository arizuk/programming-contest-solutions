n = STDIN.gets.to_i
t, a = STDIN.gets.strip.split(" ").map(&:to_i)
hs = STDIN.gets.strip.split(" ").map(&:to_i)
ds = hs.map { |h| (a - (t - h * 0.006)).abs() }.map.with_index { |v, i| [v,i] }.sort_by { |v, i| v }
puts ds[0][1] + 1