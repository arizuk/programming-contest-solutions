s = STDIN.gets.strip;

dict = [
  "dream",
  "dreamer",
  "erase",
  "eraser",
].map(&:reverse)
s = s.reverse()

i = 0
while i < s.length
  ok = false
  dict.each do |str|
    if s.slice(i, str.length) == str
      i = i + str.length
      ok = true
      break
    end
  end

  if !ok
    puts "NO"
    exit
  end
end
puts "YES"