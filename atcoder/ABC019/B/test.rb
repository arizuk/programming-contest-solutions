require 'set'

s = STDIN.gets.strip
set = {}

for i in 0..s.length-1
  for j in i+1..s.length-1
    tmp = ''
    tmp += s[0..i-1] if i >= 1
    tmp += s[i..j].reverse
    tmp += s[j+1..-1]

    unless tmp.length == s.length
      raise "#{tmp} #{tmp.length}"
    end

    if set[tmp]
      p [i, j, tmp, set[tmp]]
    end

    set[tmp] = [i, j]
  end
end

puts set.length

cnt = {}
for i in 0..s.length-1
  cnt[s[i]] ||= 0
  cnt[s[i]] += 1
end
p cnt