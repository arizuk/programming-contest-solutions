def bs(a, t)
  index = binary_serach(a, t)
  if index
    puts "index=#{index}"
  end
end

def binary_serach(a, t)
  puts "#search #{t} in #{a}"

  s = 0
  e = a.length - 1

  loop do
    i = (s + e) / 2
    # puts "#{s} - #{i} - #{e}"
    # puts "#{a[s]} .. #{a[i]} .. #{a[e]}"
    puts " mid element is #{a[i]}"

    if a[i] == t
      return i
    elsif a[i] < t
      puts " Change search range #{a[i+1]} .. #{a[e]}"
      s = i + 1
    else
      puts " Change search range #{a[s]} .. #{a[i-1]}"
      e = i - 1
    end
    break if s > e
  end
  puts " Not found"
  nil
end

bs([1], 1)
bs([1], 2)
bs([1, 5], 1)
bs([1, 5], 5)
bs([1, 5], 2)
bs([1, 5], 10)
bs([1, 2, 3, 5, 10], 6)
bs([1, 2, 3, 5, 10, 20, 100], 3)
bs([1, 2, 3, 5, 10, 20, 100], 22)
bs([1, 2, 5, 10, 30, 20, 50, 100], 3)


a = Array.new(100).map { rand(1..300) }.sort

10.times do
  bs(a, rand(1..300))
end