def print_even(n, r4)
  puts "YES"
  h = n/2
  for i in 0..n-1
    for j in 0..n-1
      if i<n/2 && j<n/2
        x = i
        y = j
      end

      if i>=n/2 && j<n/2
        x = i%h
        x = h-x-1
        y = j
      end

      if i<n/2 && j>=n/2
        x = i
        y = j%h
        y = h-y-1
      end

      if i>=n/2 && j>=n/2
        x = i%h
        x = h-x-1
        y = j%h
        y = h-y-1
      end

      print(r4[x*h+y].to_s)
      print(" ") unless j == n-1
    end
    puts
  end
end

def print_odd(n, r4, r2, r1)
  puts "YES"
  c = n/2
  h = n/2

  r2_1 = r2[0..n/2-1]
  r2_2 = r2[n/2..-1]
  for i in 0..n-1
    for j in 0..n-1
      if i == c || j == c
        if i == c && j == c
          print r1
        elsif i == c
          if j < c
            print r2_1[j]
          else
            y = (j-1)%h
            y = h-y-1
            print r2_1[y]
          end

        elsif j == c
          if i < c
            print r2_2[i]
          else
            y = (i-1)%h
            y = h-y-1
            print r2_2[y]
          end
        end
      else

        if i<c && j<c
          x = i
          y = j
        end

        if i>c && j<c
          x = (i-1)%h
          x = h-x-1
          y = j
        end

        if i<c && j>c
          x = i
          y = (j-1)%h
          y = h-y-1
        end

        if i>c && j>c
          x = (i-1)%h
          x = h-x-1
          y = (j-1)%h
          y = h-y-1
        end

        print(r4[x*h+y].to_s)
      end
      print(" ") unless j == n-1
    end
    puts
  end
end


def main()
  n = STDIN.gets.to_i
  aa = STDIN.gets.strip.split.map(&:to_i)
  counts = {}
  aa.each do |a|
    counts[a] ||= 0;
    counts[a] += 1;
  end

  center = nil
  if n%2 == 0
    unless counts.all? { |k, v| v%4==0 }
      return puts "NO"
    end

    r4 = []
    counts.each do |k, v|
      (v/4).times do
        r4.push(k)
      end
    end
    print_even(n, r4)
  else
    # (4*n-1) + 2*(n-1) + 1
    r1 = counts.keys.select { |k| counts[k] % 2 == 1 }
    unless r1.size == 1
      puts "NO"
      return
    end
    r1 = r1[0]
    counts[r1] -= 1

    # すべて偶数
    unless counts.all? { |k, v| v%2==0 }
      return puts "NO"
    end

    r4_num = ((n-1)/2) ** 2
    r4 = []
    r2 = []

    keys = counts.keys.sort_by { |k| counts[k] }.reverse
    idx = 0
    while idx <= keys.length-1 do
      k = keys[idx]
      if counts[k] == 0
        idx+=1
        next
      end

      if r4.length < r4_num
        if counts[k]%4==0
          counts[k] -= 4
          r4.push(k)
          next
        end
      end

      counts[k] -= 2
      r2.push(k)
    end

    if r4.length < r4_num
      puts "NO"
      return
    end
    print_odd(n, r4, r2, r1)
  end
end

main