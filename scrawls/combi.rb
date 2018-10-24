def combi2(s, e, k, res, resi)
  for i in s...e
    if k == 1
      res[resi] = i
      yield res
    else
      combi2(i+1, e, k-1, res, resi+1) do |res|
        res[resi] = i
        yield res
      end
    end
  end
end

def combi(s, e, k)
  for i in s...e
    if k == 1
      yield [i]
    else
      combi(i+1, e, k-1) do |ncombi|
        yield [i].concat(ncombi)
      end
    end
  end
end

# combi(0, 7, 4) do |cmb|
#   p cmb
# end

res = Array.new(4)
combi2(0, 7, 4, res, 0) do |cmb|
  p cmb
end