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
    let root = (n as f64).sqrt() as usize;
    let mut nums: Vec<_> = (1..root+1).map(|_|1).collect();
    {
        let mut cur = root + 1;
        while cur <= n {
            let lower = cur;
            let upper = n / (n/cur);
            nums.push((upper - lower + 1));
            cur = upper + 1;
        }
    }
    let nn = nums.len();
    let mut dp = vec![vec![0; nn]; k];

    for i in 0..nn {
        dp[0][i] = nums[i];
    }
    for i in 0..k-1 {
        let mut acm = 0;
        for j in (0..nn).rev() {
            acm += dp[i][nn-j-1];
            acm %= MOD;
            dp[i+1][j] = acm * nums[j] % MOD;
        }
    }
    let mut ans = 0;
    for i in 0..nn {
        ans += dp[k-1][i];
        ans %= MOD;
    }
    println!("{}", ans);
}
