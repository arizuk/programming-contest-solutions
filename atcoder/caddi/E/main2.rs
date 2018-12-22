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

fn f(a: usize, b: usize) -> (u32, u32) {
    if a <= b {
        let mut p = 0;
        while a*4usize.pow(p+1) <= b {
            p += 1;
        }
        (p, 0)
    } else {
        let mut p = 0;
        while a > b*4usize.pow(p) {
            p += 1;
        }
        (0, p)
    }
}

fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }

    // for &a in [2, 3, 10].iter() {
    //     for b in 1..20 {
    //         let (p1, p2) = f(a, b);
    //         println!("{},{} => {},{}", a, b, a*4usize.pow(p1), b*4usize.pow(p2));
    //     }
    // }

    // +
    let mut dp = vec![vec![0; 16]; n];
    for i in 0..16 {
        dp[n-1][i] = i;
    }
    for i in (0..n-1).rev() {
        for p in 0..16 {
            let a1 = aa[i];
            let a2 = aa[i+1];
            let (p1, p2) = f(a1, a2);
            let mut x = p2;
            if p > p1 {
                x += p - p1;
            }
            if x < 16 {
                dp[i][p as usize] = dp[i+1][x as usize] + p as usize;
                // debug!(a1, a2, p1, p2, i, p, i+1, x);
            } else {
                dp[i][p as usize] = dp[i+1][15] + p as usize + ((x as usize -15) * (n-i-1)) as usize;
            }
        }
    }

    // -
    let mut aa = aa;
    aa.reverse();
    for i in 0..n {
        aa[i] = aa[i] * 2;
    }

    let mut dp2 = vec![vec![0; 16]; n];
    for i in 0..16 {
        dp2[n-1][i] = i;
    }
    for i in (0..n-1).rev() {
        for p in 0..16 {
            let a1 = aa[i];
            let a2 = aa[i+1];
            let (p1, p2) = f(a1, a2);
            let mut x = p2;
            if p > p1 {
                x += p - p1;
            }
            if x < 16 {
                dp2[i][p as usize] = dp2[i+1][x as usize] + p as usize;
                // println!("dp[{}][{}] => dp[{}][{}] p1={} p2={}", i, p, i+1, x, p1, p2);
            } else {
                dp2[i][p as usize] = dp2[i+1][15] + p as usize + ((x as usize -15) * (n-i-1)) as usize;
            }
        }
    }

    let mut ans = min(dp[0][0] * 2, n + dp2[0][0]*2);
    for i in 1..n {
        let c1 = dp[i][0] * 2;
        let c2 = dp2[n-i][0] * 2 + i;
        ans = min(c1+c2, ans);
    }
    println!("{}", ans);
}
