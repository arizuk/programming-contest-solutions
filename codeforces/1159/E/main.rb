class Node
  attr_accessor :next_node, :v

  def initialize(v)
    @v = v
  end

  def add(nd)
    if next_node
      next_node.add(nd)
    else
      self.next_node = nd
    end
  end
end

T = STDIN.gets.to_i
T.times do
  n = STDIN.gets.to_i
  nxs = STDIN.gets.strip.split.map(&:to_i)

  nodes = {}
  nxs.each.with_index(1) do |nx, i|
    nodes[i] ||= Node.new(i)
    if nx == -1
    else
      nodes[nx] ||= Node.new(nx)
      nodes[nx].add(nodes[i])
    end
  end

  ans = []
  nd = nodes[n+1]
  perm = n+1
  while nd
    ans.push([nd.v, perm])
    perm -= 1
    nd = nd.next_node
  end
  ans = ans.sort_by { |v| v[0] }.map { |v| v[1] }
  ans.pop
  puts ans.join(" ")
end