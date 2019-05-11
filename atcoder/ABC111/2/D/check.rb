m = STDIN.gets.to_i
ds = STDIN.gets.strip.split.map(&:to_i)
loop do
  ss = STDIN.gets&.strip&.split(//)
  break unless ss
  x = 0
  y = 0
  ss.each_with_index do |s, i|
    case s
    when 'L'
      x -= ds[i]
    when 'R'
      x += ds[i]
    when 'D'
      y -= ds[i]
    when 'U'
      y += ds[i]
    end
  end
  puts "#{x} #{y}"
end