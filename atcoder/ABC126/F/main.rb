# m,k = STDIN.gets.strip.split.map(&:to_i)
def solve(m, k)
  ans = []
  (2**m).times do |i|
    ans.push(i)
    ans.push(i)
  end

  ans.permutation(ans.length) do |perms|
    ok = true

    for i in 0...perms.length
      for j in i+1...perms.length
        if perms[i] == perms[j]
          xor = perms[i]
          for x in i+1..j
            xor ^= perms[x]
          end

          if xor != k
            ok = false
            break
          end
        end
      end
      break unless ok
    end

    if ok
      puts "m=#{m} k=#{k} OK"
      p perms
      break
    end
  end
end

for m in 1..2
  for k in 0..10
    solve(m, k)
  end
end