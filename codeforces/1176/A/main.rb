require 'prime'

q = STDIN.gets.to_i
q.times do
    n = STDIN.gets.to_i
    divs = Prime.prime_division(n)
    ok = true
    divs.each do |n, cnt|
        if n > 5
            puts "-1"
            ok = false
            break
        end
    end
    next unless ok
    divs = divs.to_h
    ans = divs[2] || 0;
    ans += (divs[3] || 0) * 2;
    ans += (divs[5] || 0) * 3;
    puts ans
end