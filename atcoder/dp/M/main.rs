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

#[derive(Debug,Clone)]
pub struct BIT {
    bits: Vec<u64>,
}
impl BIT {
    pub fn new(size: usize) -> Self {
        let mut bit_size = 1;
        while bit_size <= size {
            bit_size *= 2;
        }
        BIT {
            bits: vec![0; bit_size + 1],
        }
    }
    pub fn sum(&mut self, i: usize) -> u64 {
        let mut i = i as i64;
        let mut ret = 0;
        while i > 0 {
            ret += self.bits[i as usize];
            ret %= MOD;
            i -= i & -i;
        }
        ret
    }
    pub fn add(&mut self, i: usize, value: u64) {
        assert!(i > 0);
        let n = self.bits.len() as i64;
        let mut i = i as i64;
        while i <= n {
            self.bits[i as usize] += value;
            self.bits[i as usize] %= MOD;
            i += i & -i;
        }
    }
}

fn main() {
    input!{
      n: usize,
      k: usize,
      aa: [usize; n],
    }
    let mut dp = vec![BIT::new(k+1); n+1];
    dp[0].add(k+1, 1);

    for i in 0..n {
        for j in 1..k+2 {
            let upper = min(j+aa[i], k+1);
            let val = dp[i].sum(upper) + MOD - dp[i].sum(j-1);
            dp[i+1].add(j, val%MOD);
        }
    }
    println!("{}", dp[n].sum(1) % MOD);
}
