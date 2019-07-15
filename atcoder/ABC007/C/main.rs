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
use std::io::{stdout, stdin, BufWriter, Write};

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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      r: usize,
      c: usize,
      s: (usize1,usize1),
      g: (usize1,usize1),
      cs: [chars; r]
    }

    const INF: usize = 1 << 31;
    let mut dist = vec![vec![INF; c]; r];
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((s.0, s.1, 0));
    while q.len() > 0 {
        let (x, y, d) = q.pop_front().unwrap();
        if dist[x][y] <= d {
            continue;
        }
        dist[x][y] = d;
        for (nx, ny) in next_points(x, y, r, c) {
            if cs[nx][ny] == '.' {
                let n_dist = dist[x][y] + 1;
                if n_dist < dist[nx][ny] {
                    q.push_back((nx, ny, n_dist));
                }
            }
        }
    }
    let ans = dist[g.0][g.1];
    puts!("{}", ans);
}
