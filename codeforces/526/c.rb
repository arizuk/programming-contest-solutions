MOD = 10**9 + 7
ss = STDIN.gets.strip
chunks = ss.split(/b+/).map do |s|
  cnt = 0
  for i in 0..(s.length-1)
    cnt += 1 if s[i] == 'a'
  end
  cnt
end
ans = 1
chunks.each do |n|
  ans *= n+1
  ans %= MOD
end
puts (ans-1) % MOD