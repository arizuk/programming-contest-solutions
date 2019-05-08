class Integer
  def digits
    Enumerator.new do |x|
      to_s.chars.map{|c| x << c.to_i }
    end
  end
end

N,K = STDIN.gets.strip.split.map(&:to_i)
a = STDIN.gets.strip.split.map(&:to_i)

ans = 1 << 31
a.combination(K).each do |comb|
  p = comb.reduce(&:+)
  num = 0
  p.digits.each do |d|
    num += d/5 + d%5
  end
  ans = [ans, num].min
end
puts ans