N = 6
R = 10
X = [1, 7, 15, 20, 30, 50]

i = 0
m = 0
while i < N
  s = X[i]
  i += 1 while (i < N && X[i] - s <= R)
  p = X[i - 1]
  i += 1 while (i < N && X[i] - p <= R)
  m += 1
end
puts m