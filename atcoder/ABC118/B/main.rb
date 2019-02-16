n,m = STDIN.gets.strip.split.map(&:to_i)

fav = Array.new(35) { 0 }

n.times do
  aa = STDIN.gets.strip.split.map(&:to_i)
  k = aa.shift
  for a in aa
    fav[a]+=1
  end
end

ans = 0
for i in 0..35
  ans += 1 if fav[i] == n
end
puts ans