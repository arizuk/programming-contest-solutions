weeks = ["SUN","MON","TUE","WED","THU","FRI","SAT"]

ss = STDIN.gets.strip

i = 0
index = 0
weeks.each do |w|
  if ss == w
    index = i
    break
  end
  i += 1
end

if index == 0
  puts 7
else
  puts 7-index
end