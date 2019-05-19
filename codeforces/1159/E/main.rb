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

def ok?(ps, nxs)
  n = nxs.length
  q = []
  (n-1).downto(0) do |i|
    q.pop while !q.empty? && ps[q[-1]] < ps[i]
    return false if q.empty? && nxs[i] != n+1
    return false if !q.empty? && nxs[i] != q[-1]+1
    q.push(i)
  end
end

T = STDIN.gets.to_i
T.times do
  n = STDIN.gets.to_i
  nxs = STDIN.gets.strip.split.map(&:to_i)
  for i  in 0..n
    nxs[i] = i+2 if nxs[i] == -1
  end

  nodes = {}
  nxs.each.with_index(1) do |nx, i|
    nodes[i] ||= Node.new(i)
    # nx = i+1 if nx == -1
    nodes[nx] ||= Node.new(nx)
    nodes[nx].add(nodes[i])
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
  if ok?(ans, nxs)
    puts ans.join(" ")
  else
    puts -1
  end
end