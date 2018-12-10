def dfs(edges, curr, visited = {})
  visited[curr] = true
  next_edges = edges[curr] || []
  next_edges.each do |node|
    next if visited[node]
    dfs(edges, node, visited)
  end
  visited
end

def bridge?(n, abs, i)
  edges = {}
  abs.each_with_index do |ab, j|
    next if j == i
    a, b = ab
    edges[a] ||= []
    edges[b] ||= []
    edges[a] << b
    edges[b] << a
  end
  visited = dfs(edges, abs[0][0])
  visited.keys.length < n
end

n, m = STDIN.gets.split(" ").map(&:to_i)

abs = []
m.times do
  a, b = STDIN.gets.split(" ").map(&:to_i)
  abs << [a, b]
end

ans = 0
for i in 0...m
  ans += 1 if bridge?(n, abs, i)
end
puts ans
