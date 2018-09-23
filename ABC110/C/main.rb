s = STDIN.gets().strip();
t = STDIN.gets().strip();

dict1 = {}

for i in 0...s.length do
  dict1[s[i]] ||= 0
  dict1[s[i]] += 1
end

dict2 = {}
for i in 0...t.length do
  dict2[t[i]] ||= 0
  dict2[t[i]] += 1
end

if dict1.values.sort == dict2.values.sort
  puts "Yes"
else
  puts "No"
end