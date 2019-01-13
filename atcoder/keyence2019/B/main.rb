 s = STDIN.gets.chomp

 for i in 0..s.length-1
  for l in 0..s.length-1
    ss = s.dup
    ss.slice!(i, l)
    if ss == 'keyence'
      puts "YES"
      return
    end
  end
 end
 puts "NO"