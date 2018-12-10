def gcd(a, b)
  return a if (b == 0)
  gcd(b, a % b)
end

N, X = STDIN.gets.split(" ").map(&:to_i)
x = STDIN.gets.split(" ").map(&:to_i)
x.push(X)
x = x.sort
x = x.map.with_index(0) do |x_i, i|
  i == x.length - 1 ? nil : x[i+1] - x[i]
end
x.pop
d = x[0]
for i in 1..(x.length-1) do
  d = gcd(x[i], d)
end
puts d