N = STDIN.gets.to_i

seen = {}
success = true
prev = nil
N.times do
  word = STDIN.gets.strip

  if seen[word]
    success = false
    break
  end
  seen[word] = true

  if prev
    if prev[-1] != word[0]
      success = false
      break
    end
  end
  prev = word
end

if success
  puts 'Yes'
else
  puts 'No'
end