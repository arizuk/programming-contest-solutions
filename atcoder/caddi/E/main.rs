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

fn count(aa: &Vec<u64>, n: usize) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0; 16]; n+1];
    for i in 0..16 {
        dp[n-1][i] = i;
    }
    for i in (0..n-1).rev() {
        for p in 0..16 {
            let a1 = aa[i]*4u64.pow(p as u32);
            let mut a2 = aa[i+1];
            let mut x = 0;
            while a2 < a1 {
                x += 1;
                a2 *= 4;
            }
            let p = p as usize;
            let x = x as usize;
            if x < 16 {
                dp[i][p] = dp[i+1][x] + p;
            } else {
                dp[i][p] = dp[i+1][15] + p + (x - 15) * (n-i-1);
            }
        }
    }
    dp
}

fn main() {
    input!{
      n: usize,
      aa: [u64; n],
    }

    let dp = count(&aa, n);
    let mut aa = aa;
    aa.reverse();
    let dp2 = count(&aa, n);

    let mut ans: usize = 1 << 60;
    for i in 0..n+1 {
        let c1 = dp[i][0] * 2;
        let c2 = dp2[n-i][0] * 2 + i;
        ans = min(c1+c2, ans);
    }
    println!("{}", ans);
}
