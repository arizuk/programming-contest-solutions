N = 4
W = [2, 1, 3, 2]
V = [3, 2, 4, 2]
WEIGHT = 5

DP = Array.new(N+1).map { Array.new(WEIGHT+1, -1) }

# i番目以降のアイテムから、重さの総和がj以下となるように選ぶ
def rec(i, j)
  if DP[i][j] >= 0
    return DP[i][j]
  end

  if i == N
    res = 0
  # 入らない
  elsif j < W[i]
    res = rec(i + 1, j)
  else
    res = [rec(i + 1, j), rec(i + 1, j - W[i]) + V[i]].max
  end

  return DP[i][j] = res
end

p rec(0, WEIGHT)