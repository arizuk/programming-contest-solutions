H, W = STDIN.gets.split(" ").map(&:to_i)
a = []
H.times do
  a << STDIN.gets.split(" ").map(&:to_i)
end

# def dump(a)
#   puts "#" * 10
#   a.each do |r|
#     puts r.join(" ")
#   end
# end
# dump(a)

order = []
for i in 0...H do
  for j in 0...(W-1) do
    next if a[i][j] % 2 == 0
    a[i][j] -= 1
    a[i][j+1] += 1
    order << [i, j, i, j+1].map { |i| i + 1 }.join(" ")
  end
end

for i in 0...(H-1) do
  j = W-1
  next if a[i][j] % 2 == 0
  a[i][j] -= 1
  a[i+1][j] += 1
  order <<  [i, j, i+1, j].map { |i| i + 1 }.join(" ")
end
# dump(a)

puts order.size
puts order.join("\n")