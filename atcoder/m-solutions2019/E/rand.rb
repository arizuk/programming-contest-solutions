MOD = 10**6 + 3;
q = 2
puts q + 3
q.times do
    puts [rand(MOD/2), rand(MOD/2), rand(MOD+1000)].join(" ")
end
puts [0, 1, 2].join(" ")
puts [1, 0, 2].join(" ")
puts [1, 4, 1].join(" ")