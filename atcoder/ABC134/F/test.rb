cnt = {}
for i in 1..4
  for j in 1..4
    next if i == j
    for k in 1..4
      next if k == i || k == j
      for l in 1..4
        next if l == i || l == j || l == k
        v = (i-1).abs() + (j-2).abs() + (k-3).abs() + (l-4).abs()
        p [i, j, k, l, v]
        cnt[v] ||= 0
        cnt[v] += 1;
      end
    end
  end
end

for i in 0..16
  p [i, cnt[i]]
end
