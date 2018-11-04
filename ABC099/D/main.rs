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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn main() {
    input!{
      n: usize,
      c: usize,
      ds: [[usize; c]; c],
      cs: [[usize; n]; n],
    }
    let mut cost = vec![vec![0; c+1]; 3];
    for i in 0..n {
        for j in 0..n {
            for c1 in 1..c+1 {
                if cs[i][j] != c1 {
                    cost[(i + j + 2) % 3][c1] += ds[cs[i][j]-1][c1-1];
                }
            }
        }
    }

    let mut ans = 1 << 30;
    for c1 in 1..c+1 {
        for c2 in 1..c+1 {
            for c3 in 1..c+1 {
                if c1 == c2 || c2 == c3 || c3 == c1 {
                    continue;
                }
                let cost1 = cost[0][c1] + cost[1][c2] + cost[2][c3];
                ans = min(cost1, ans);
            }
        }
    }
    println!("{}", ans);
}
