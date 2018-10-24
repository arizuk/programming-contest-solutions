maze = <<EOF
#S######.#
......#..#
.#.##.##.#
.#........
##.##.##.#
....####.#
.####.....
....#.###.
....#...G#
EOF

MAZE = maze.split("\n").map do |s|
  s.split("").map do |v|
    case v
    when 'S' then 0
    when '.' then 1
    when '#' then 2
    when 'G' then 3
    end
  end
end
MIN = MAZE.map { |m| Array.new(m.size, nil) }

start = nil
for i in 0...MAZE.length
  for j in 0...MAZE[i].length
    if MAZE[i][j] == 0
      MIN[i][j] = 0
      start = [i, j]
      break
    end
  end
  break if start
end

def print_m
  MIN.each do |r|
    puts r.map { |d| "%02s" % (d || '--') }.join(" ")
  end
end

def bfs(sx, sy)
  dir = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1],
  ]
  queue = []
  queue << [sx, sy]
  loop do
    if queue.length == 0
      print_m
      raise "ERROR"
    end

    x, y = queue.shift
    # puts "x=#{x}, y=#{y}"
    d = MIN[x][y]

    dir.each do |dx, dy|
      nx = x + dx
      ny = y + dy

      next if nx < 0 || nx >= MAZE.length || ny < 0 || ny >= MAZE[0].length
      next if MIN[nx][ny]
      next unless MAZE[nx][ny] == 1 || MAZE[nx][ny] == 3

      MIN[nx][ny] = d + 1
      queue.push([nx, ny])
      return d + 1 if MAZE[nx][ny] == 3
      # p [nx, ny]
    end
  end
end

puts bfs(*start)
print_m