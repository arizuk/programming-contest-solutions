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
const MOD: usize = 1e9 as usize + 7;

fn main() {
    input!{
      n: usize,
    }
    let mut dp = vec![vec![vec![vec![0; 4]; 4]; 4]; n];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let t = [k, j, i];
                if t == [0, 1, 2] { continue; } // agc
                if t == [1, 0, 2] { continue; } // gac
                if t == [0, 2, 1] { continue; } // acg
                dp[0][i][j][k] = 1;
            }
        }
    }

    for i in 1..n-2 {
        for c1 in 0..4 {
            for c2 in 0..4 {
                for c3 in 0..4 {
                    for c4 in 0..4 {
                        let t = [c3, c2, c1];
                        if t == [0, 1, 2] { continue; } // agc
                        if t == [1, 0, 2] { continue; } // gac
                        if t == [0, 2, 1] { continue; } // acg
                        // A?GC
                        if c4 == 0 && c1 == 2 && c2 == 1 { continue; }
                        // AG?C
                        if c4 == 0 && c1 == 2 && c3 == 1 { continue; }

                        dp[i][c1][c2][c3] += dp[i-1][c2][c3][c4];
                        dp[i][c1][c2][c3] %= MOD;
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for c1 in 0..4 {
        for c2 in 0..4 {
            for c3 in 0..4 {
                ans += dp[n-3][c1][c2][c3];
                ans %= MOD;
            }
        }
    }
    println!("{}", ans);
}
