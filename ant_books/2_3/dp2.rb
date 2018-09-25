N = 4
W = [2, 1, 3, 2]
V = [3, 2, 4, 2]
WEIGHT = 5

DP = Array.new(N+1).map { Array.new(WEIGHT+1, 0) }

def solve
  (N-1).step(0, -1) do |i|
    for j in 0..WEIGHT
      if j < W[i]
        DP[i][j] = DP[i + 1][j]
      else
        DP[i][j] = [DP[i + 1][j], DP[i+1][j-W[i]] + V[i]].max
      end
    end
  end
  puts DP[0][WEIGHT];
end

solve
