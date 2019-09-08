for n in 2..15
  for x in 2..(2*n-2)
    input = "#{n} #{x}\n"
    File.write("cases/input#{n}_#{x}", input)
  end
end