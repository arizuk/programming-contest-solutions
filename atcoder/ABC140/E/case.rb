n = rand(10) + 2
a = (1..n).to_a.shuffle

ans = 0
# p a
for i in 0...n-1
  nums = []
  nums << a[i]
  for j in i+1...n
    nums << a[j]
    nums.sort!
    nums.reverse!
    ans += nums[1]
    # p [i, j, nums[1], nums]
  end
end


input = <<"EOL"
#{n}
#{a.join(" ")}
EOL

output = <<"EOL"
#{ans}
EOL

# puts input
# puts output

File.write("input_test", input)
File.write("output_test", output)
