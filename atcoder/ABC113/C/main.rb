n, m = STDIN.gets.strip.split.map(&:to_i)
pys = []
m.times do
  pys << STDIN.gets.strip.split.map(&:to_i)
end

ans = {}
pys.sort_by { |v| v[1] }.each do |p, y|
  ans[p] ||= []
  ans[p] << y
end
yans = {}
ans.each do |pref, ys|
  ys.each_with_index do |y, i|
    yans[y] = "%06d%06d" % [pref, i+1]
  end
end
pys.each do |p, y|
  puts yans[y]
end