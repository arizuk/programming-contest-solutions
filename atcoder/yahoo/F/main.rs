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
      s: chars,
    }
    let n = s.len();
    let mut dp = vec![vec![0; 2001]; 2001];
    dp[0][0] = 1;
    let mut r = 0;
    let mut b = 0;

    const MOD: usize = 998244353;

    for i in 0..n*2 {
        if i < n {
            match s[i] {
                '0' => {
                    r += 2;
                },
                '1' => {
                    r += 1;
                    b += 1;
                },
                '2' => {
                    b += 2;
                },
                _ => {}
            }
        }

        let k = i+1;
        for nr in 0..k+1 {
            let nb = k-nr;
            if !(nr <= r && nb <= b) {
                continue;
            }
            // debug!(i, nr, nb);

            if nr > 0 {
                dp[nr][nb] += dp[nr-1][nb];
            }
            if nb > 0 {
                dp[nr][nb] += dp[nr][nb-1];
            }
            dp[nr][nb] = dp[nr][nb] % MOD;
        }
    }

    let mut ans = 0;
    for nr in 0..2*n+1 {
        let nb = 2*n - nr;
        ans += dp[nr][nb];
        ans %= MOD;
    }
    println!("{}", ans);
}
