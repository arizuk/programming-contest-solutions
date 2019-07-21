def ans(n)
  for a in 1..n
    for b in 1..n
      if a*a + b*b == n
        return [a, b]
      end
    end
  end
  false
end

for i in 2..100
  temp = ans(i)
  p [i, temp]
end