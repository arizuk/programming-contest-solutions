require 'prime'

N = STDIN.gets.to_i

primes = {}
for i in 2..N
  divs = Prime.prime_division(i)
  divs.each do |n, pow|
    primes[n] ||= 0
    primes[n] += pow
  end
end

ans = []

n24 = []
n14 = []
n2 = []
n4 = []
primes.each do |n, pow|
  if pow >= 74
    ans.push([[n, 74]])
  end

  if pow >= 24
    n24.push(n)
  end

  if pow >= 14
    n14.push(n)
  end

  if pow >= 4
    n4.push(n)
  end

  if pow >= 2
    n2.push(n)
  end
end

# 25*3
n24.each do |v1|
  n2.each do |v2|
    if v1 != v2
      ans.push([[v1, 24], [v2, 2]])
    end
  end
end

# 15*5
n14.each do |v1|
  n4.each do |v2|
    if v1 != v2
      ans.push([[v1, 14], [v2, 4]])
    end
  end
end

# 5*5*3
n4.each do |v1|
  n4.each do |v2|
    next unless v1 < v2
    n2.each do |v3|
      if v1 != v2 && v2 != v3 && v1 != v3
        # puts v1**4 * v2**4 * v3**2
        ans.push([[v1, 4], [v2, 4], [v3, 2]])
      end
    end
  end
end

puts ans.length