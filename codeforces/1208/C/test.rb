def check(t)
  n = t.length-1
  v1 = (0..n).reduce(0) { |sum, j| sum ^ t[0][j] }
  for i in 0..n
    ans = (0..n).reduce(0) { |sum, j| sum ^ t[i][j] }
    if v1 != ans
      raise "error #{i} #{v1} != #{ans}"
    end

    ans = (0..n).reduce(0) { |sum, j| sum ^ t[j][i] }
    if v1 != ans
      raise "error #{i} columns #{v1} != #{ans}"
    end
  end
  v1
end


def make_table(n)
  t = Array.new(n) { Array.new(n, 0) }

  cur = 0
  for c in 0...(n/2)
    for r in 0...(n/2)
      t[2*r][2*c] = cur
      t[2*r][2*c+1] = cur+1
      t[2*r+1][2*c] = cur+2
      t[2*r+1][2*c+1] = cur+3
      cur += 4
    end
  end
  t
end

for i in 1..5
  v = check(make_table(4*i))
  puts "#{i} #{v}"
end

t = make_table(12)
t.each do |r|
  p r
end
