n = rand(10) + 1
a = (1..n).to_a.shuffle

ans = []
for i in 0...a.length
  sum = 0
  for j in 0...i
    if a[j] < a[i]
      sum += a[j]
    end
  end
  ans << sum
end

input = <<"EOL"
#{n}
#{ans.join(" ")}
EOL

output = <<"EOL"
#{a.join(" ")}
EOL



File.write("input_test", input)
File.write("output_test", output)
