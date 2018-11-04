k = STDIN.gets.to_i

i = 1
d = 1
while k > 0
  puts i

  i += d;
  if i >= 10*d
    d *= 10
    i = 2*d-1
  end

  k -= 1
end