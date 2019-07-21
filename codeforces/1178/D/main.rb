require 'prime'

for n in 3..1000
  cur = n
  loop do
    if Prime.prime?(cur)
      rem = cur - n;
      if rem > n/2
        p [n, cur]
      else
        # p "ok"
      end
      break
    end
    cur += 1
  end
end