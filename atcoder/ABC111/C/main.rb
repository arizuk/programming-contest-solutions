n = STDIN.gets.to_i
vs = STDIN.gets.strip.split(" ").map(&:to_i)

def count(a)
  cnt = {}
  a.each { |v| cnt[v] ||= 0; cnt[v] += 1 }
  cnt.sort_by { |v, cnt| cnt }.reverse
end

a = []
b = []
vs.each_with_index do |v, i|
  if i % 2 == 0
    a << v
  else
    b << v
  end
end

ca = count(a)
ca.push([0, 0]) if ca.length == 1
cb = count(b)
cb.push([0, 0]) if cb.length == 1

if ca[0][0] == cb[0][0]
  ans = n - [
    ca[0][1] + cb[1][1],
    ca[1][1] + cb[0][1],
  ].max
else
  ans = n - ca[0][1] - cb[0][1]
end
puts ans