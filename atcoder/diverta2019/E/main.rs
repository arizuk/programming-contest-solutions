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

fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }
    let mut acm = vec![0; n+1];
    for i in 0..n {
        acm[i+1] = acm[i] ^ aa[i];
    }

    const SZ: usize = 1 << 20;
    let mut dp0 = vec![1; SZ];
    let mut dp1 = vec![0; SZ];
    let mut cnt = vec![0; SZ];

    const MOD: usize = 1e9 as usize + 7;

    let mut z = 0;
    for i in 1..n+1 {
        let a = acm[i];
        if a == 0 {
            z += 1;
        } else {
            let p0 = dp0[a];
            let p1 = dp1[a];
            dp0[a] = (p0 + (p1 * ((z - cnt[a]) % MOD))) % MOD;
            dp1[a] = (dp0[a] + p1) % MOD;
            cnt[a] = z;
        }
    }

    if acm[n] == 0 {
        let mut ans = 1;
        for _ in 0..z-1  {
            ans *= 2;
            ans %= MOD;
        }
        for i in 0..dp1.len() {
            ans += dp1[i];
            ans %= MOD;
        }
        println!("{}", ans);
    } else {
        println!("{}", dp0[acm[n]]);
    }
}
