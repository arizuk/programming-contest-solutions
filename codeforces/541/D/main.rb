N,M = STDIN.gets.strip.split.map(&:to_i)


class Group
  attr_reader :next_grp, :prev_grp

  def initialize()
    @values = {}
  end

  def add(val)
    @values[val] = true
  end

  def remove(val)
    @values.delete(val)
  end

  def next_grp
    @next_grp ||= Group.new
  end

  def prev_grp
    @prev_grp ||= Group.new
  end
end

class Equal < Base
end

N.times do |i|
  reviews = STDIN.gets.strip.split

  val = "n#{i}"
  list[0].add(val)
  n_grp = list[0]

  for i in 0..M-1
    m = "m#{i}"
    m_grp = nil
    review = reviews[i]

    if review == '>'
      m_grp = n_grp.prev_grp
      m_grp.add(m)
    elsif review == '<'
      m_grp = n_grp.next_grp
      m_grp.add(m)
    else
      grp.add(m)
    end
  end
end