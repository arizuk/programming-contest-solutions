USED = Array.new(5, false)
PERM = Array.new(5, 0)

def permutation(pos, n)
  if pos == n
    p PERM
    return
  end

  for i in 0...n do
    unless USED[i]
      PERM[pos] = i
      USED[i] = true
      permutation(pos+1, n)
      USED[i] = false
    end
  end
end

permutation(0, 5)