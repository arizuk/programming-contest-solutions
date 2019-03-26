# @param {Integer[]} nums
# @return {Void} Do not return anything, modify nums in-place instead.
def next_permutation(nums)
  n = nums.size

  for i in 1...n
    i = n-i-1
    if nums[i] < nums[i+1]
      for j in 0...n
        j = n-j-1
        if nums[i] < nums[j]
          # swap
          temp = nums[i]
          nums[i] = nums[j]
          nums[j] = temp
          break
        end
      end
      # reverse elements after i
      reverse(nums, i+1)
      return
    end
  end

  reverse(nums, 0)
end

def reverse(array, i)
  head = i
  tail = array.length-1

  while head < tail
    temp = array[head]
    array[head] = array[tail]
    array[tail] = temp
    head += 1
    tail -= 1
  end
end

# array = [1, 2, 3, 4]
# 10.times do
#   next_permutation(array)
#   p array
# end