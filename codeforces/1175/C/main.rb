puts 5
5.times do
    n = rand(100)
    a = []
    n.times do
        a.push(rand(10**4 + 1))
    end
    a.uniq!
    a = a.sort()
    n = a.length
    k = rand(n)
    puts "#{n} #{k}"
    puts a.join(" ")
end

a = [1,2,5,8,11,12,25]
k = 4
for x in 1..25
    ds = a.map { |v| (v-x).abs() }
    ds = ds.sort()
    p [x, ds[k]]
end