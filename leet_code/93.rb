# @param {String} s
# @return {String[]}
def restore_ip_addresses(s)
  list = []
  for i in 1..3
    for j in i+1..(i+3)
      for k in j+1..(j+3)
        r1 = s[0...i]
        r2 = s[i...j]
        r3 = s[j...k]
        r4 = s[k..-1]
        if [r1, r2, r3, r4].all? { |r| r.to_i <= 255 && r.to_i.to_s == r }
          list << [r1, r2, r3, r4].join(".")
        end
      end
    end
  end
  list
end

# p restore_ip_addresses("25525511135")