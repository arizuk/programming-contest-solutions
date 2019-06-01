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

const MOD: usize = 1e6 as usize + 3;

#[allow(dead_code)]
pub struct ModFactorial {
    fact: Vec<usize>,
    inv: Vec<usize>,
    finv: Vec<usize>,
}
impl ModFactorial {
    pub fn new(max_value: usize) -> Self {
        let mut fact = vec![0; max_value + 1];
        let mut inv = vec![0; max_value + 1];
        let mut finv = vec![0; max_value + 1];
        fact[0] = 1;
        fact[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..max_value + 1 {
            fact[i] = fact[i - 1] * i % MOD;
            inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }
        ModFactorial {
            fact: fact,
            inv: inv,
            finv: finv,
        }
    }
    pub fn combination(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        self.fact[n] * self.finv[n - k] % MOD * self.finv[k] % MOD
    }
}

pub fn mod_pow(b: usize, p: usize) -> usize {
    if p == 0 {
        return 1;
    }
    let mut ret = mod_pow(b * b % MOD, p / 2) % MOD;
    if p % 2 == 1 {
        ret = ret * b % MOD;
    }
    ret
}
pub fn inv(a: usize) -> usize {
    mod_pow(a, MOD - 2)
}

fn main() {
    input!{
      q: usize,
      xdns: [(usize,usize,usize); q],
    }
    let fact = ModFactorial::new(MOD);
    for (x, d, n) in xdns {
        if d==0 {
            println!("{}", mod_pow(x, n));
            continue;
        }
        if x == 0 {
            println!("{}", 0);
            continue;
        }
        let x = x * inv(d) % MOD;
        if x + n - 1 >= MOD {
            println!("{}", 0);
        } else {
            let ans = fact.fact[x+n-1] * fact.finv[x-1] % MOD * mod_pow(d, n) % MOD;
            println!("{}", ans);
        }
    }
}
