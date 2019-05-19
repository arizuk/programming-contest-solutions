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

fn main() {
    input!{
      k: chars,
      d: usize,
    }
    const MOD: usize = 1e9 as usize + 7;
    let n = k.len();
    let mut dp = vec![vec![vec![0; d]; 2]; n+1];
    dp[0][0][0] = 1;

    for i in 0..n {
        let c = (k[i] as u8 - '0' as u8) as usize;
        for flag in 0..2 {
            for m in 0..d {
                for dig in 0..10 {
                    if dig < c {
                        dp[i+1][1][(dig + m) % d] += dp[i][flag][m];
                        dp[i+1][1][(dig + m) % d] %= MOD;
                    } else if dig == c {
                        dp[i+1][flag][(dig + m) % d] += dp[i][flag][m];
                        dp[i+1][flag][(dig + m) % d] %= MOD;
                    } else {
                        if flag == 1 {
                            dp[i+1][flag][(dig + m) % d] += dp[i][flag][m];
                            dp[i+1][flag][(dig + m) % d] %= MOD;
                        }
                    }
                }
            }
        }
    }
    let ans = MOD + dp[n][1][0] + dp[n][0][0] - 1; // exclude 0
    println!("{}", ans%MOD);
}
