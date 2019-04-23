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

const MOD: usize = 998244353;

fn add(a:usize, b:usize) -> usize {
    (a+b) % MOD
}

fn pow3(n: usize) -> usize {
    let mut ret = 1;
    for _ in 0..n {
        ret *= 3;
        ret %= MOD;
    }
    ret
}

fn mul(a: usize, b: usize) -> usize {
    (a*b) %MOD
}

fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }
    let sum: usize = aa.iter().sum();
    let mut dp = vec![vec![0; sum+1]; n+1];
    dp[0][0] = 1;

    let mut dp2 = vec![vec![0; sum+1]; n+1];
    dp2[0][0] = 1;

    for i in 0..n {
        for s in 0..sum+1 {
            dp[i+1][s] = add(dp[i+1][s], dp[i][s] * 2); // B or C
            if s+aa[i] <= sum {
                dp[i+1][s+aa[i]] = add(dp[i+1][s+aa[i]], dp[i][s]);
            }

            dp2[i+1][s] = add(dp2[i+1][s], dp2[i][s]); // B
            if s+aa[i] <= sum {
                dp2[i+1][s+aa[i]] = add(dp2[i+1][s+aa[i]], dp2[i][s]);
            }
        }
    }
    debug!(dp[n].iter().sum::<usize>());
    debug!(dp2[n].iter().sum::<usize>());

    let half = (sum+1)/2;
    let mut ng = 0;
    for i in half..sum+1 {
        ng = add(ng, dp[n][i]);
    }
    if sum%2 == 0 {
        ng = add(ng, MOD - dp2[n][sum/2]);
    }
    debug!(ng);
    ng = mul(ng, 3);

    let mut ans = pow3(n);
    ans = add(ans, MOD - ng);
    println!("{}", ans);
}
