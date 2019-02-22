N = 7

def check(u, v)
  for a in 0..N
    for b in 0..N
      if a+b == v && a^b == u
        p [u, v, a, b]
        return true
      end
    end
  end
  false
end

ans = 0
for u in 0..N
  for v in 0..N
    if check(u, v)
      ans += 1
    end
  end
end
puts ans