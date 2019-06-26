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
use std::collections::BinaryHeap;
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

const INF: usize = 1 << 50;

type Pair = (usize,usize);
fn solve(ss: &Vec<Vec<char>>, x: usize, h:usize,w:usize, s:Pair,g:Pair) -> usize {
    let mut dist = vec![vec![INF; w]; h];
    let mut q = BinaryHeap::new();
    q.push(
        (Rev(0),s)
    );
    dist[s.0][s.1] = 0;

    while q.len() > 0 {
        let (Rev(d), (cx, cy)) = q.pop().unwrap();
        if dist[cx][cy] < d {
            continue;
        }

        for (nx, ny) in next_points(cx, cy, h, w) {
            let nx_dist = dist[cx][cy] + if ss[nx][ny] == '#' { x } else { 1 };
            if dist[nx][ny] > nx_dist {
                dist[nx][ny] = nx_dist;
                q.push((Rev(nx_dist), (nx, ny)));
            }
        }
    }
    dist[g.0][g.1]
}

#[doc = " [l, r)"]
pub fn binary_search_by<F>(mut l: usize, mut r: usize, f: &F) -> usize
where
    F: Fn(usize) -> bool,
{
    assert!(l <= r);
    while r != l {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r
}

fn main() {
    input!{
      h: usize,
      w: usize,
      t: usize,
      ss: [chars; h],
    }

    let mut s = (0,0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == 'S' {
                s = (i, j);
            }
            if ss[i][j] == 'G' {
                g = (i, j);
            }
        }
    }

    let f = |x| {
        let d = solve(&ss, x, h, w, s, g);
        d > t
    };
    let ans = binary_search_by(1, t, &f);
    println!("{}", ans-1);
}
