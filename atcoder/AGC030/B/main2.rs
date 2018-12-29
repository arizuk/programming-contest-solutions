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
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;

fn main() {
    input!{
      l: usize,
      n: usize,
      xs: [usize; n],
    }

    let mut dp = vec![vec![vec![0; 2]; n+1]; n+1];
    let mut cs = vec![vec![vec![vec![0;2];2]; n]; n];

    // u,dからupする場合、downする場合
    for u in 0..n {
        for d in 0..n {
            if u+d>=n { break; }

            // [u][d][0] up
            let mut x = xs[u];
            if u > 0 {
                x = x - xs[u-1];
            }
            cs[u][d][0][0] = x;

            let mut x = 0;
            if u > 0 {
                x = xs[u-1];
            }
            let dx = xs[n-d-1];
            cs[u][d][0][1] = x + (l-dx);

            // [u][d][1] down
            let mut x = l;
            if d > 0 {
                x = xs[n-d];
            }

            cs[u][d][1][0] = (l-x) + xs[u];
            cs[u][d][1][1] = x - xs[n-d-1];

            if u == 0 && d > 0 {
                cs[u][d][0][0] = 0;
                cs[u][d][0][1] = 0;
            }
            if u > 0 && d == 0 {
                cs[u][d][1][0] = 0;
                cs[u][d][1][1] = 0;
            }
        }
    }

    for u in 0..n+1 {
        for d in 0..n+1 {
            if u+d>n { break; }
            if u > 0 {
                dp[u][d][0] = max(
                        dp[u-1][d][0] + cs[u-1][d][0][0],
                        dp[u-1][d][1] + cs[u-1][d][1][0]
                );
            }
            // when down
            if d > 0 {
                dp[u][d][1] = max(
                    dp[u][d-1][0] + cs[u][d-1][0][1],
                    dp[u][d-1][1] + cs[u][d-1][1][1]
                );
            }
        }
    }

    let mut ans = 0;
    for u in 0..n+1 {
        let d = n-u;
        ans = max(ans, max(dp[u][d][0], dp[u][d][1]));
    }
    println!("{}", ans);
}
