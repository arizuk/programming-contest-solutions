for i in 2..16
  for j in i..16
    if j%i == j^i
      p [i, j, j%i, j^i]
    end
  end
end