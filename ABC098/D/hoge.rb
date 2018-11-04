1.upto(9) do |a|
  1.upto(9) do |b|
    1.upto(9) do |c|
      add = a + b + c
      xor = a ^ b ^ c
      if add == xor
        puts "%05d" % a.to_s(2)
        puts "%05d" % b.to_s(2)
        puts "%05d" % c.to_s(2)
        puts "#{a} #{b} #{c} = #{add} #{xor}"
      end
    end
  end
end