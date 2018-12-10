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
const MOD : i64 = 1e9 as i64 + 7;

fn main() {
    input!{
      h: usize,
      w: usize,
      k: usize,
    }
    let mut dp = vec![vec![0; w+1]; h+1];
    dp[0][1] = 1;
    for hi in 0..h {
        for wi in 1..w+1 {
            if dp[hi][wi] == 0 { continue; }
            for bit in 0..(1 << w - 1) {
                let mut ok = true;
                for i in 0..w as i64 -2 {
                    if bit >> i & 1 == 1 && bit >> i+1 & 1 == 1 { ok = false };
                }
                if ok {
                    // println!("{:03b}", bit);
                    if wi < w && 1 << ((wi - 1)) & bit > 0 {
                        dp[hi+1][wi+1] = (dp[hi+1][wi+1] + dp[hi][wi]) % MOD;
                    } else if wi > 1 && (1 << (wi - 2)) & bit > 0 {
                        dp[hi+1][wi-1] = (dp[hi+1][wi-1] + dp[hi][wi]) % MOD;
                    } else {
                        dp[hi+1][wi] = (dp[hi+1][wi] + dp[hi][wi]) % MOD;
                    }
                }
            }
        }
    }
    println!("{}", dp[h][k]);
}
