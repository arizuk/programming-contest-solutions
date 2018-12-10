n = STDIN.gets.strip.to_i

def check(n)
  d = n % 10
  while n > 0
    if n % 10 != d
      return false
    end
    n = n / 10
  end
  true
end


while true
  if check(n)
    puts n
    exit
  end
  n = n + 1
end