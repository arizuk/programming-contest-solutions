n = STDIN.gets.to_i
ans = 0
for i in 0..n do
  cnt = 0
  for j in 1..i do
    cnt += 1 if i % j == 0 && i % 2 == 1
  end
  ans += 1 if cnt == 8
end
puts ans
