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
      k: usize,
    }
    let rn = (n as f64).sqrt() as usize;

    let mut lus = vec![0; rn+1];
    for i in 1..rn+1 {
        let upper = n/i;
        let lower = max(n/(i+1) + 1, rn+1);
        if lower <= upper {
            lus[i] = upper - lower + 1;
            // debug!(i, lower, upper);
        }
    }

    let mut dp1 = vec![vec![0; rn+2]; k];
    let mut dp2 = vec![vec![0; rn+2]; k];

    for j in 1..rn+1 {
        dp1[0][j] = 1;
        dp2[0][j] = lus[j];
    }

    for i in 0..k-1 {
        let mut acm = vec![0; rn+2];
        for j in 1..rn+1 {
            acm[j+1] = acm[j] + dp1[i][j];
            acm[j+1] %= MOD;
        }

        let mut acm2 = vec![0; rn+2];
        for j in (1..rn+1).rev() {
            acm2[j-1] = acm2[j] + dp2[i][j] % MOD;
            acm2[j-1] %= MOD;
        }


        for j in 1..rn+1 {
            dp1[i+1][j] += acm[rn+1];
            dp1[i+1][j] %= MOD;
            dp1[i+1][j] += acm2[j-1];
            dp1[i+1][j] %= MOD;

            dp2[i+1][j] += acm[j+1] * lus[j] % MOD;
            dp2[i+1][j] %= MOD;
        }
    }

    // debug!(lus);
    // for i in 0..k {
    //     debug!(dp1[i]);
    //     debug!(dp2[i]);
    // }

    let mut ans = 0;
    for i in 1..rn+1 {

        ans += dp1[k-1][i];
        ans %= MOD;

        ans += dp2[k-1][i];
        ans %= MOD;
    }
    println!("{}", ans);
}
