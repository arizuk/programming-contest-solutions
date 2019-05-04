require 'logger'
logger = Logger.new('logfile.log')

def output(msg)
  puts msg
  STDOUT.flush
end

def init_rem
  rem = {}
  for c in 'A'..'E'
    rem[c] = 24
  end
  rem
end

def resolve(rems)
  ans = Array.new(4)
  5.times do
    rems.each_with_index do |rs, i|
      rs = rs.reject { |c| ans.include?(c) }
      if rs.size == 1
        ans[i] = rs[0]
      end
    end
  end

  for c in 'A'..'E'
    if !ans.include?(c)
      ans.push(c)
      return ans.join("")
    end
  end
end

t,f = STDIN.gets.strip.split.map(&:to_i)
t.times do
  i = 0
  base = 0

  rem = init_rem
  rems = []

  f.times do
    i += 1
    if i == 119
      i = 1
      base += 1
      rems << rem.keys
      rem = init_rem
    end

    nth = base + 1 + (5*(i-1))
    output nth
    letter = STDIN.gets.strip
    logger.info([i, base, nth, rem, letter])
    rem[letter] -= 1
    if rem[letter] == 0
      rem.delete(letter)
    end
  end

  ans = resolve(rems)
  logger.info([rems, ans])

  output resolve(ans)
  STDOUT.flush
  result = STDIN.gets.strip
  if result != 'Y'
    break
  end
end