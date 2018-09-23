require 'prime'

def combi(n, a, index)
  if n == 0
    p a
    return
  end
  for i in 1..n do
    a[index..index+n] = 1
    combi(n - i, a, index+i)
    a[index..index+n] = 0
  end
end

factors = 60.prime_division
a = []
factors.each do |v, pow|
  pow.times { a << v }
end
# combi(a, 0, a.length-1)
combi(5, Array.new(5, 0), 0)