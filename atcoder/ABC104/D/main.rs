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

const MOD: usize = 1000000007;

fn add(a: usize, b: usize) -> usize {
    (a + b) % MOD
}

fn solve(s: &[char]) -> usize {
    let mut dp = vec![vec![0; 4]; s.len()+1];
    dp[s.len()][3] = 1;
    for i in (0..s.len()).rev() {
        for j in 0..4 {
            if s[i] == '?' {
                dp[i][j] = add(dp[i][j], 3 * dp[i+1][j]);
            } else {
                dp[i][j] = add(dp[i][j], dp[i+1][j]);
            }
        }
        if s[i] == '?' || s[i] == 'A' { dp[i][0] = add(dp[i][0], dp[i+1][1]) };
        if s[i] == '?' || s[i] == 'B' { dp[i][1] = add(dp[i][1], dp[i+1][2]) };
        if s[i] == '?' || s[i] == 'C' { dp[i][2] = add(dp[i][2], dp[i+1][3]) };
    }
    return dp[0][0];
}

fn main() {
    input!{
      ss: chars,
    }
    let ans = solve(&ss);
    println!("{}", ans);
}
