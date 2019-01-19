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

const INF: i64 = 1 << 60;

fn rec(dp: &mut Vec<Vec<i64>>, aa: &Vec<i64>, i: usize, j: usize) -> i64 {
    let n = aa.len();
    if i + j >= n {
        return 0;
    }
    if dp[i][j] != INF {
        return dp[i][j];
    }

    let ans;
    if (i+j)%2 == 0 {
        ans = max(
            rec(dp, aa, i+1, j) + aa[i],
            rec(dp, aa, i, j+1) + aa[n-j-1]
        )
    } else {
        ans = min(
            rec(dp, aa, i+1, j) - aa[i],
            rec(dp, aa, i, j+1) - aa[n-j-1]
        )
    }
    dp[i][j] = ans;
    ans
}

fn main() {
    input!{
      n: usize,
      aa: [i64; n],
    }
    let mut dp = vec![vec![INF; n]; n];
    println!("{}", rec(&mut dp, &aa, 0, 0));
}
