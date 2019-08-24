n = 10
puts n
cur = 0
for i in 0...n
  puts cur
  cur += 1
  if rand(5) == 1
    cur = 2
  end
end