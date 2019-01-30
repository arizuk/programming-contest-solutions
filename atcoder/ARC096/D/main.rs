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

fn solve(xvs: &[(i64, i64)], c: i64) -> i64 {
    let n = xvs.len();

    let mut dp = vec![0; n];
    {
        let mut sum = 0;
        for i in (0..n).rev() {
            let (x, v) = xvs[i];
            sum += v;
            sum -= c-x;
            if i < n-1 {
                sum += c-xvs[i+1].0;
                dp[i] = max(dp[i+1], sum);
            } else {
                dp[i] = max(0, sum);
            }
        }
    }

    let mut ans = 0;
    let mut sum = 0;
    for i in 0..n {
        let (x, v) = xvs[i];
        sum += v;
        sum -= x;
        if i > 0 {
            sum += xvs[i-1].0;
        }
        ans = max(ans, sum);

        if i < n-1 {
            let sum2 = sum - x + dp[i+1];
            ans = max(ans, sum2);
        }
    }
    ans
}


fn main() {
    input!{
      n: usize,
      c: i64,
      xvs: [(i64, i64); n],
    }
    let mut ans = solve(&xvs, c);
    let mut xvs: Vec<(i64, i64)> = xvs.into_iter().map(|(x, v)| ((c-x), v)).collect();
    xvs.reverse();
    ans = max(solve(&xvs, c), ans);
    println!("{}", ans);
}
