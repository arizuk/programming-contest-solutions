def f(b, n)
  return n if n < b
  f(b, n/b) + n%b
end

# for b in 2..5
#   ans = f(b, 10)
#   p [b, 10, ans]
# end

for b in 2..20
  ans = f(b, 28)
  p [b, 28, ans]
end

# p [10, 87654, f(10, 87654)]
