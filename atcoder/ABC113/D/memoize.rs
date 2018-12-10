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

fn path(w: usize, a: usize, b: usize) -> i64 {
    let mut i = 0;
    let mut cnt = 0;

    while i < (1 << w-1) {
        let mut ok = true;
        {
            let mut j = i;
            let mut last = 0;
            while j > 0 {
                if last == 1 && j % 2 == 1 {
                    ok = false;
                    break;
                }
                last = j%2;
                j /= 2;
            }
        }
        if !ok {
            i += 1;
            continue;
        }

        if a == b {
            if (1 << (a + 1 - 2)) & i > 0 {
                i += 1;
                continue;
            }
            if a >= 2 && (1 << (a - 2)) & i > 0 {
                i += 1;
                continue;
            }
            cnt += 1;
        } else {
            if (1 << (b - 2)) & i > 0 {
                cnt += 1;
            }
        }
        i += 1;
    }
    cnt
}

fn solve(h: usize, w: usize, k: usize, hi: usize, wi: usize, dp: &mut Vec<Vec<i64>>) -> i64 {
    if dp[hi][wi] != -1 {
        return dp[hi][wi];
    }
    assert!(wi >= 1 && wi <= w);
    let mut ans = solve(h, w, k, hi+1, wi, dp) * path(w, wi, wi) % MOD;
    if wi > 1 {
        ans = (ans + solve(h, w, k, hi+1, wi - 1, dp) * path(w, wi-1, wi)) % MOD;
    };
    if wi < w {
        ans = (ans + solve(h, w, k, hi+1, wi + 1, dp) * path(w, wi, wi+1)) % MOD;
    };
    dp[hi][wi] = ans % MOD;
    return ans % MOD;
}

fn main() {
    input!{
      h: usize,
      w: usize,
      k: usize,
    }
    let mut dp = vec![vec![-1; w+1]; h+2];
    for i in 0..w+1 {
        dp[h+1][i] = 0;
    }
    dp[h+1][k] = 1;
    println!("{}", solve(h, w, k, 1, 1, &mut dp));
}
