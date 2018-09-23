n, m, xc, yc = STDIN.gets.split(" ").map(&:to_i)
xs = STDIN.gets.split(" ").map(&:to_i)
ys = STDIN.gets.split(" ").map(&:to_i)

no = false
for z in (xc+1)..(yc) do
  ok = xs.all? { |x| x < z }
  next unless ok
  ok = ys.all? { |y| y >= z }
  next unless ok
  puts "No War"
  no = true
  break
end
puts "War" unless no
