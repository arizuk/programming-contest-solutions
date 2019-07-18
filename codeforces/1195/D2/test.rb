g = 1
x = 2
y = 3
z = 59

16.times do |i|
  p [i, g]
  g = (g * x + y) % z
end