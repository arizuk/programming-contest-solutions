ans = []
for i in 1..10
    for j in 1..10
        for k in 1..10
            if i * j <= 10 && j * k <= 10
                # puts [i, j, k].join(" ")
                ans.push([i, j, k])
            end
        end
    end
end

cnt = {}
ans.each { |v| cnt[v[2]] ||= 0; cnt[v[2]] += 1; }
p cnt
# ans = ans.sort_by { |v| v[2] }
# ans.each do |a|
#     puts a.join(" ")
# end