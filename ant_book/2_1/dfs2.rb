A = [1, 2, 3, 4]
T = 7

def dfs(i, sum)
  return sum == T if i == A.length
  return true if dfs(i + 1, sum)
  return true if dfs(i + 1, sum + A[i])
  false
end

puts "Search #{T}"
puts dfs(0, 0) ? "Yes" : "No"
