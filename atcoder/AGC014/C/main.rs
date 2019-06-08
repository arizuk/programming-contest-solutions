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

    let mut visited = vec![vec![false; w]; h];
    let mut start = (0,0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i,j);
                break;
            }
        }
    }
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, start.0, start.1));
    visited[start.0][start.1] = true;
    let mut ans = h*w + 1;
    while let Some((c, x, y)) = q.pop_front() {
        let d = min(min(x, y), min(h-x-1, w-y-1));
        let num = (d + k - 1) /k + 1;
        ans = min(ans, num);
        if c >= k {
            continue;
        }
        for (nx, ny) in next_points(x, y, h, w) {
            if s[nx][ny] != '#' && !visited[nx][ny] {
                visited[nx][ny] = true;
                q.push_back((c+1, nx, ny));
            }
        }
    }
    println!("{}", ans);
}