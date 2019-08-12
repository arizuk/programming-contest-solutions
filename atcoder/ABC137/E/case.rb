n = rand(5) + 1
m = rand(n * (n-1))
p = rand(100)

seen = {}
prev = 1
es = []
for i in 2...n
  if rand(2) >= 1
    seen[[prev,i]] = true
    es << [prev, i, rand(100)]
    prev = i
  end
end
es << [prev, n, rand(100)]
seen[[prev,n]] = true

m = [m, es.length].max

puts [n, m, p].join(" ")
m -= es.length

while m > 0
  loop do
    a = rand(n) + 1
    b = rand(n) + 1

    unless seen[[a, b]]
      es << [a, b, rand(100)]
      seen[[a,b]] = true
      break
    end
  end
  m -= 1
end

es.each { |v| puts v.join(" ") }