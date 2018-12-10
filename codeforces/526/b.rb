n, s = STDIN.gets.strip.split.map(&:to_i)
vs = STDIN.gets.strip.split.map(&:to_i)

sum = vs.reduce(&:+)
if sum < s
  puts "-1"
else
  puts (sum-s)/n
end