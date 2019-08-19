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
      n: usize,
      k: usize,
      aa: [usize; n],
    }
    let mut dp = vec![vec![None; 2]; 42];
    dp[41][0] = Some(0);

    for d in (0..41).rev() {
        let bit = 1 << d;
        let mut n1 = 0;
        for i in 0..n {
            if aa[i] & bit > 0 {
                n1 += 1;
            }
        }
        let n0 = n - n1;

        if k & bit > 0 {
            dp[d][0] = dp[d+1][0].map(|v| v + n0 * bit); // 1->1
            dp[d][1] = dp[d+1][0].map(|v| v + n1 * bit); // 1->0
        } else {
            dp[d][0] = dp[d+1][0].map(|v| v + n1 * bit);
        }
        dp[d][1] = max(dp[d][1], dp[d+1][1].map(|v| v + bit * max(n0, n1)));
    }
    let ans = max(dp[0][0], dp[0][1]);
    println!("{}", ans.unwrap());
}