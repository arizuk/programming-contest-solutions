n = STDIN.gets.to_i
a = STDIN.gets.strip.split.map(&:to_i)

ans = 0
temp = {}
# p a
for i in 0...n-1
  nums = []
  nums << a[i]
  for j in i+1...n
    nums << a[j]
    nums.sort!
    nums.reverse!
    ans += nums[1]
    p [i, j, nums[1], nums]
    temp[nums[1]] ||= 0
    temp[nums[1]] += nums[1]
  end
end

p ans
keys = temp.keys.sort
keys.each do |k|
  p [k, temp[k]]
end