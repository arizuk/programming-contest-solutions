def calc(n)
  x = 1
  y = n - 1
  loop do
    # puts "x=#{x}, y=#{y}"
    x += 1
    ny = n%x==0 ? (n-1)/x : n/x
    y = y <= ny ? y - 1 : ny
    break if y <= 0
  end
  x-1
end

100.times do |i|
  puts "#{i+1} => #{calc(i+1)}"
end