aa = STDIN.gets.strip.split.map(&:to_i)
n = aa.length

temp = []
for i in 0...n
  a = aa[i]
  idx = temp.find_index { |t| t == a }
  if idx
    if idx == 0
      temp = []
    else
      temp = temp[0..idx-1]
    end
  else
    temp.push(a)
  end
  p [i, temp]
end