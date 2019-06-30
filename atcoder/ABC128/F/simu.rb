a,b = STDIN.gets.strip.split.map(&:to_i)

ans = Array.new(50) { '-' }
cur = 0;
3.times do |i|
    ans[cur] = '+'
    cur = cur + a
    raise "muri" if ans[cur] == '+'
    ans[cur] = '+'

    if i < 2
        cur = cur - b
        raise "muri" if ans[cur] == '+'
        ans[cur] = '+'
    end
end

puts ans[0..cur].join(" ")