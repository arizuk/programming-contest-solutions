def ok(mid)
  mid >= 0
end

def bsearch(l, r)
  while r>l
    mid = (r+l)/2
    if ok(mid)
      r = mid
    else
      break if l == mid
      l = mid
    end
  end
  r
end


p bsearch(0, 1)
p bsearch(0, 3)
p bsearch(0, 8)
p bsearch(0, 9)
p bsearch(0, 10)
p bsearch(0, 11)