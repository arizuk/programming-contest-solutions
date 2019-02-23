class Base
  attr_accessor :parent
  def root
    parent ? parent.root : self
  end
end

class Integer
  def size
    1
  end

  def parent=(a)
  end
end

class Node < Base
  attr_reader :size, :items
  def initialize(items)
    @size = items.map(&:size).reduce(&:+)
    @items = items
    @items.each do |item|
      item.parent = item
    end
  end

  def add(item)
    @items << item
    @size += item.size
    item.parent = self
    self
  end

  def merge(b)
    if size >= b.size
      add(b)
    else
      b.add(self)
    end
  end
end

N = STDIN.gets.to_i
map = Array.new(N) { nil }

(N-1).times do |i|
  a,b = STDIN.gets.strip.split.map(&:to_i)
  a -= 1
  b -= 1
  map[a] ||= Node.new([a])
  map[b] ||= Node.new([b])
  map[a].root.merge(map[b].root)
end

def print_node(node)
  case node
  when Integer
    print("#{node+1} ")
  else
    node.items.each do |item|
      print_node(item)
    end
  end
end

print_node(map[0].root)
puts