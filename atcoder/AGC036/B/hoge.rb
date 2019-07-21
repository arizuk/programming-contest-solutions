n,k = STDIN.gets.strip.split.map(&:to_i)
aa = STDIN.gets.strip.split.map(&:to_i)


def solve(n, k, aa)
  temp = []
  k.times do
    for i in 0...n
      a = aa[i]
      idx = temp.find_index { |t| t == a }
      if idx
        if idx == 0
          temp = []
        else
          temp = temp[0..idx-1]
        end
      else
        temp.push(a)
      end
    end
  end
  p [n, k, temp]
end

p aa
for k in 1..10
  solve(n, k, aa)
end