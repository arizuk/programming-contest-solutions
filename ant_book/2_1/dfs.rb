A = [1, 2, 4, 7]
T = 15
STACK = []

def search
  sum = STACK.sum
  if sum == T
    p STACK
    return true
  end
  return false if sum > T

  for i in 0..(A.length-1) do
    next unless A[i]
    STACK.push(A[i])
    A[i] = nil

    if search
      return true
    else
      A[i] = STACK.pop
    end
  end
  false
end

puts "Search #{T}"
puts search ? "Yes" : "No"
