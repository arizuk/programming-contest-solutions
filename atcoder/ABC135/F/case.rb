slen = rand(6) + 1
tlen = rand(7) + 1

s = (0...slen).map { (rand(4) + 97).chr }.join
t = (0...tlen).map { (rand(4) + 97).chr }.join

puts s
puts t