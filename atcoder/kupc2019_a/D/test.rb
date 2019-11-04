def calc(n)
  ans = 0
  nums = (1..n*2).to_a
  nums.combination(n).each do |p1|
    p2 = (1..n*2).to_a.select { |v| !p1.include?(v) }

    ok = true
    for i in 0...n
      if p1[i] > p2[i]
        ok = false
      end
    end
    if ok
      ans += 1
    end
  end
  ans
end

def combi(n, k)
  ret = 1
  k.times do |i|
    ret *= (n-i)
  end

  k.times do |i|
    ret /= (i+1)
  end

  ret
end

for n in 1..5
  p [n, calc(n), combi(2*n, n)]
end