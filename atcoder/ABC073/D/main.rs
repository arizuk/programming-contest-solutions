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

fn permutation(pos: usize, n: usize, used: &mut [bool], dp: &[Vec<usize>], rs: &[usize], last: usize, dist: usize) -> usize {
    if pos == n {
        return dist;
    }

    let mut cost = 1 << 32;
    for i in 0..n as usize {
        if used[i] == false {
            used[i] = true;
            let next_dist = if last == n  {0} else { dp[rs[i]][rs[last]] };
            cost = min(cost, permutation(pos + 1, n, used, dp, rs, i, dist + next_dist));
            used[i] = false;
        }
    }
    cost
}

fn main() {
    input!{
      n: usize,
      m: usize,
      r: usize,
      rs: [usize; r],
      abcs: [(usize, usize, usize); m],
    }
    const INF: usize = 1 << 32;

    let mut dp = vec![vec![INF; n+1]; n+1];
    for &(a, b, c) in &abcs {
        dp[a][b] = c;
        dp[b][a] = c;
    }
    for k in 0..n+1 {
        for i in 0..n+1 {
            for j in 0..n+1 {
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }
    // debug!(dp);

    let mut used = vec![false; r];
    let ans = permutation(0, r, &mut used, &dp, &rs, r, 0);
    println!("{}", ans);
}
