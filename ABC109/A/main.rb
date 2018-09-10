def check(a, b)
  3.times do |i|
    ret = a * b * (i + 1)
    return true if ret % 2 == 1
  end
  false
end

a, b = STDIN.gets.split(" ").map(&:to_i)

if check(a, b)
  puts "Yes"
else
  puts "No"
end