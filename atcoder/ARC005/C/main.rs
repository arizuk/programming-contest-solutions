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

fn main() {
    input!{
      h: usize,
      w: usize,
      board: [chars; h],
    }

    const INF: usize = 1 << 32;
    let mut costs = vec![vec![INF; w]; h];
    let mut s = (0,0);
    let mut g = (0,0);
    for h in 0..h {
        for w in 0..w {
            if board[h][w] == 's' {
                s = (h,w);
            } else if board[h][w] == 'g' {
                g = (h,w);
            }
        }
    }

    // Start point
    costs[s.0][s.1] = 0;

    // ダイクストラ
    // Dijkstra's algorithm
    use std::collections::BinaryHeap;
    let mut q = BinaryHeap::new();
    q.push((Rev(0),s));

    while q.len() > 0 {
        let (Rev(cur_cost), (x,y)) = q.pop().unwrap();
        if costs[x][y] < cur_cost { // <= のが良さそう
            continue;
        }

        for (nx, ny) in next_points(x, y, h, w) {
            let mut next_cost = 0;
            if board[nx][ny] == '#' {
                next_cost = 1;
            }

            if costs[x][y] + next_cost < costs[nx][ny] {
                costs[nx][ny] = costs[x][y] + next_cost;
                q.push(
                    (Rev(costs[nx][ny]), (nx, ny))
                );
            }
        }
    }

    if costs[g.0][g.1] <= 2 {
        println!("{}", "YES");
    } else {
        println!("{}", "NO");
    }
}
