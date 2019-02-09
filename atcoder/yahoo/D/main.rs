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
      aa: [usize; l],
    }
    let mut dp = vec![vec![0; 5]; l+1];
    for i in 0..5 { dp[0][i] = 0; }

    for i in 0..l {
        dp[i+1][0] = dp[i][0] + aa[i];

        let even = if aa[i] == 0 { 2 } else { aa[i]%2 };
        let odd = if aa[i]%2 == 1 { 0 } else { 1 };

        dp[i+1][1] = min( dp[i][0], dp[i][1] ) + even;
        dp[i+1][2] = min(dp[i][0], min(dp[i][1], dp[i][2])) + odd;
        dp[i+1][3] = min(dp[i][0], min(dp[i][1], min(dp[i][2], dp[i][3]))) + even;
        dp[i+1][4] = [dp[i][0], dp[i][1],dp[i][2],dp[i][3],dp[i][4]].iter().min().unwrap() + aa[i];
    }
    // debug!(dp);
    let ans = dp[l].iter().min().unwrap();
    println!("{}", ans);
}
