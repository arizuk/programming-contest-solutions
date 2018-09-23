require 'prime'

def mod_combi(n, m)
  return mod_combi(n, n-m) if n-m < m;

  ans = 1
  for i in 0...m do
    ans *= n - i;
    ans /= i + 1;
  end
  # puts "#{n} #{m} -> #{ans}"
  ans
end

mod = 10**9 + 7;
n, m = STDIN.gets.split(" ").map(&:to_i)
factors = m.prime_division

ans = 1
factors.each do |v, pow|
  ans *= mod_combi(n+pow-1, n-1)
end
puts ans % mod

