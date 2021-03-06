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

fn rec(dp: &mut Vec<Vec<Vec<f64>>>, n: usize, i: usize, j: usize, k: usize) -> f64 {
    if dp[i][j][k] > 0f64 {
        return dp[i][j][k];
    }
    if i == 0 && j == 0 && k == 0 {
        return 0.0;
    }
    let mut ans = 0.0 as f64;
    if i > 0 { ans += rec(dp, n, i-1, j, k) * i as f64; }
    if j > 0 { ans += rec(dp, n, i+1, j-1, k) * j as f64; }
    if k > 0 { ans += rec(dp, n, i, j+1, k-1) * k as f64; }

    ans += n as _;
    ans *= 1.0 / (i + j + k) as f64;

    dp[i][j][k] = ans;
    ans
}

fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }

    let mut xx = 0;
    let mut yy = 0;
    let mut zz = 0;
    for i in 0..n {
        if aa[i] == 1 { xx += 1; }
        if aa[i] == 2 { yy += 1; }
        if aa[i] == 3 { zz += 1; }
    }
    let mut dp = vec![vec![vec![-1.0; n+1]; n+1]; n+1];
    dp[0][0][0] = 0.0;
    println!("{}", rec(&mut dp, n, xx, yy, zz));
}
