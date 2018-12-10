s = STDIN.gets.strip
k = STDIN.gets.to_i
n = s.length

strs = []
for i in 0...n
  for j in i..i+5
    strs << s[i..j]
  end
end
puts strs.sort.uniq[k-1]
