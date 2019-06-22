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
const MOD: u64 = 1e9 as u64 + 7;

pub fn mod_pow(b: u64, p: u64) -> u64 {
    if p == 0 {
        return 1;
    }
    let mut ret = mod_pow(b * b % MOD, p / 2) % MOD;
    if p % 2 == 1 {
        ret = ret * b % MOD;
    }
    ret
}

fn main() {
    input!{
      n: usize,
      x: u64,
      y: u64,
      z: u64,
    }

    let d = (x+y+z) - 1;
    let size = 1 << d;
    let mut dp = vec![vec![0u64; size]; n+1];
    dp[0][0] = 1;
    for i in 0..n {
        for a in 1..11 {
            // 直前のD桁
            for bit in 0..(1 << d) {
                let nb = (bit << a) + (1 << (a-1));
                // println!("{} {:b} {:b}", a, bit, nb);

                // xyz文字列を含むのでだめ
                let xng = nb & (1 << (x+y+z-1)) > 0;
                let yng = nb & (1 << (y+z-1)) > 0;
                let zng = nb & (1 << (z-1)) > 0;
                if xng && yng && zng {
                    continue;
                }

                // D桁に絞る
                let mask = (1 << d) - 1;
                // println!("{:b} {:b}", mask, nb&mask);

                dp[i+1][nb & mask] += dp[i][bit];
                dp[i+1][nb & mask] %= MOD;
            }
        }
    }
    let mut ans = 0u64;
    for &v in dp[n].iter() {
        ans += v;
        ans %= MOD;
    }
    ans = (mod_pow(10, n as _) + MOD - ans) % MOD;
    println!("{}", ans);
}
