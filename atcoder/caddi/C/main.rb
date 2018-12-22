require 'prime'

n,p = STDIN.gets.strip.split.map(&:to_i)

divs = Prime.prime_division(p)
ans = 1
divs.each do |v, c|
  next if c < n
  ans *= v**(c/n)
end
puts ans