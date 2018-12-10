n = STDIN.gets.strip
n.length.times do |i|
  n[i] = n[i] == '9' ? '1' : '9';
end
puts n