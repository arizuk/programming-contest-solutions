def gcd(a, b)
  b == 0 ? a : gcd(b, a % b)
end

def lcm(a, b)
  a * b / gcd(a, b)
end

n, m = STDIN.gets.split(" ").map(&:to_i)
s = STDIN.gets.strip
t = STDIN.gets.strip
l = lcm(n, m)
x = Array.new(l, nil)

for i in 0...n
  j = i*l/n;
  x[j] = s[i]
end

for i in 0...m
  j = i*l/m
  if !(x[j] == nil || x[j] == t[i])
    puts "-1"
    exit
  end
end
puts l