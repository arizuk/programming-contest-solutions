#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;
use std::cmp::Ordering;

fn next_points(x:usize, y:usize, h:usize, w:usize) -> Vec<(usize,usize)> {
    let moves = [
        (0,1),
        (0,-1),
        (1,0),
        (-1,0),
    ];

    let mut nxys = vec![];
    for &(dx, dy) in moves.iter() {
        let nx = x as i64 + dx;
        let ny = y as i64 + dy;
        if !(nx >= 0 && nx < h as i64 && ny >= 0 && ny < w as i64) {
            continue;
        }
        nxys.push(((nx as usize), (ny as usize)));
    }
    nxys
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rev(usize);

impl Ord for Rev {
    fn cmp(&self, other: &Rev) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Rev {
    fn partial_cmp(&self, other: &Rev) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}


fn main() {
    input!{
      h: usize,
      w: usize,
      k: usize,
      s: [chars; h],
    }

    const INF: usize = 1 << 32;
    let mut cost = vec![vec![INF; w]; h];
    use std::collections::BinaryHeap;
    let mut q = BinaryHeap::new();
    for &i in [0, h-1].iter() {
        for j in 0..w {
            if s[i][j] == '#' {
                q.push((Rev(1), i, j));
                cost[i][j] = 1;
            } else {
                q.push((Rev(0), i, j));
                cost[i][j] = 0;
            }
        }
    }
    for &j in [0, w-1].iter() {
        for i in 0..h {
            if s[i][j] == '#' {
                q.push((Rev(1), i, j));
                cost[i][j] = 1;
            } else {
                q.push((Rev(0), i, j));
                cost[i][j] = 0;
            }
        }
    }

    while let Some((Rev(c), i, j)) = q.pop() {
        if cost[i][j] < c {
            continue;
        }
        for (nx, ny) in next_points(i, j, h, w) {
            let mut next_cost = c;
            if s[nx][ny] == '#' {
                next_cost += 1;
            }
            if cost[nx][ny] > next_cost {
                cost[nx][ny] = next_cost;
                q.push((Rev(next_cost), nx, ny));
            }
        }
    }

    let mut start = (0,0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i,j);
                break;
            }
        }
    }

    let mut visited = vec![vec![false; w]; h];

    let mut q = std::collections::VecDeque::new();
    q.push_back((0, start.0, start.1));
    visited[start.0][start.1] = true;

    debug!(cost);

    let mut ans = INF;
    while let Some((c, x, y)) = q.pop_front() {
        ans = min(ans, cost[x][y]);
        if c >= k {
            continue;
        }
        for (nx, ny) in next_points(x, y, h, w) {
            if s[ny][ny] != '#' && !visited[nx][ny] {
                visited[nx][ny] = true;
                q.push_back((c+1, nx, ny));
            }
        }
    }
    println!("{}", ans);
}