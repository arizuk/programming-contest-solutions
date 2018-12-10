n = STDIN.gets.to_i
ws = STDIN.gets.strip.split.map(&:to_i)

class Node
  attr_reader :i, :lch, :rch, :lcost, :rcost
  attr_accessor :parent, :p_cost
  def initialize(i)
    @i = i
  end

  def left(lch, lcost)
    @lch = lch
    @lcost = lcost
    @lch.parent = self
    @lch.p_cost = lcost
  end

  def right(rch, rcost)
    @rch = rch
    @rcost = rcost
    @rch.parent = self
    @rch.p_cost = rcost
  end
end


nodes = {}
(n-1).times do
  u, v, c = STDIN.gets.strip.split.map(&:to_i)
  nodes[u-1] ||= Node.new(u-1)
  nodes[v-1] ||= Node.new(v-1)

  node = nodes[u-1]
  if node.lch
    rnode = nodes[v-1]
    node.right(rnode, c)
  else
    lnode = nodes[v-1]
    node.left(lnode, c)
  end
end

RES = Array.new(n) { [nil, nil, nil] }

def search_childs(node)
  if node.lch && node.lcost <= ws[node.i]
    RES[node.i][0] = ws[node.i] - node.p_cost + search(node.parent)
  end
  if node.parent && node.p_cost <= ws[node.i]
    RES[node.i][0] = ws[node.i] - node.p_cost + search(node.parent)
  end
end