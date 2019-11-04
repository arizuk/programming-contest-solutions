a,b,x = STDIN.gets.strip.split.map(&:to_i)

if a*a*b/2 >= x
  tan = (a*b*b).to_f / (2.0 * x.to_f)
else
  tan = 2*b.to_f/a.to_f - 2*x.to_f/(a.to_f**3)
end

puts Math.atan(tan) / Math::PI * 180
