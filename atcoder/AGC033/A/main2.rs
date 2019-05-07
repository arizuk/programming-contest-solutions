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
use std::collections::VecDeque;
type I = usize;

fn main() {
    input!{
      h: usize,
      w: usize,
      mut aa: [chars; h]
    }
    let mut q = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if aa[i][j] == '#' {
                q.push_back((i, j, 1));
            }
        }
    }

    let mut ans = 0;
    while q.len() > 0 {
        let (i, j, d) = q.pop_front().unwrap();
        let dirs = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ];
        for &(dx, dy) in dirs.iter() {
            let x = (i as i64) + dx;
            let y = (j as i64) + dy;
            if !(x >= 0 && x < h as i64 && y >= 0 && y < w as i64) {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if aa[x][y] == '.' {
                aa[x][y] = '#';
                q.push_back((x, y, d+1));
                ans = max(ans, d);
            }
        }
    }
    println!("{}", ans);
}
