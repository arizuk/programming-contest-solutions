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

fn dfs(edges: &Vec<Vec<usize>>, cur: usize, par: usize, dp: &mut Vec<Vec<usize>>) {
    let mut white = 1;
    let mut black = 1;
    for &next in edges[cur].iter() {
        if par != next {
            dfs(edges, next, cur, dp);
            white *= (dp[next][0] + dp[next][1]) % MOD;
            white %= MOD;

            black *= dp[next][0];
            black %= MOD;
        }
    }
    dp[cur][0] = white;
    dp[cur][1] = black;
}


fn main() {
    input!{
      n: usize,
      xys: [(usize,usize); n-1],
    }

    let mut edges = vec![vec![]; n];
    for &(x, y) in xys.iter() {
        edges[x-1].push(y-1);
        edges[y-1].push(x-1);
    }
    let mut dp = vec![vec![0; 2]; n];
    dfs(&edges, 0, n, &mut dp);
    let ans = dp[0][0] + dp[0][1];
    println!("{}", ans%MOD);
}
