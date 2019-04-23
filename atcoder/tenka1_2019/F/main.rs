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
type I = usize;

// fn mod_3pow(n: usize) {
//     for i in 0..n {
//     }
// }

fn main() {
    input!{
      n: usize,
      x: usize,
    }
    let mut dp = vec![vec![0; x+1]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    dp[0][2] = 1;

    for i in 1..n {
        dp[i][0] = 3usize.pow(i as u32);
        dp[i][1] = 3usize.pow(i as u32);
        dp[i][2] = 3usize.pow(i as u32);
        for j in 3..x {
            dp[i][j] += dp[i-1][j];
            if j >= 1 {
                dp[i][j] += dp[i-1][j-1];
            }
            if j >= 2 {
                dp[i][j] += dp[i-1][j-2];
            }
        }
        dp[i][x] += dp[i-1][x-1];
        if x >=2 {
            dp[i][x] += dp[i-1][x-2];
        }
    }
    for i in 0..n {
        debug!(dp[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        let rem = n-i-1;
        let tmp = dp[i][x] * 3usize.pow(rem as u32);
        ans += tmp;
        debug!(i, dp[i][x], tmp);
    }
    // debug!(dp);
    ans = 3usize.pow(n as u32)- ans;
    println!("{}", ans);
}
