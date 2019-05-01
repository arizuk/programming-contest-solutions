1000.times do |t|
  v = rand(10**15)
  ans = `echo #{v}|./out`.to_i

  s1 = v.to_s.split(//).map { |v| v.to_i }.sum
  s2 = ans.to_s.split(//).map { |v| v.to_i }.sum
  if s1 != s2
    puts "ERROR #{[v, ans, s1, s2]}"
    exit
  else
    puts "OK #{[v, ans, s1, s2]}"
  end
end