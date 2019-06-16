A = rand(2000) + 1
B = rand(2000) + 1
MAX = 50
problem = [A, B].join(" ") + "\n"
problem += (1..A).map { rand(MAX)+1 }.join(" ") + "\n"
problem += (1..B).map { rand(MAX)+1 }.join(" ") + "\n"
puts problem