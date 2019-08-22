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
use std::io::{stdout, stdin, BufWriter, Write};

const MOD: u64 = 998244353;

#[allow(dead_code)]
pub struct ModFactorial {
    fact: Vec<u64>,
    inv: Vec<u64>,
    finv: Vec<u64>,
    modulo: u64,
}
impl ModFactorial {
    pub fn new(max_value: usize, modulo: u64) -> Self {
        let mut fact = vec![0; max_value + 1];
        let mut inv = vec![0; max_value + 1];
        let mut finv = vec![0; max_value + 1];
        fact[0] = 1;
        fact[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..max_value + 1 {
            fact[i] = fact[i - 1] * i as u64 % modulo;
            inv[i] = modulo - inv[modulo as usize % i] * (modulo / i as u64) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }
        ModFactorial {
            fact: fact,
            inv: inv,
            finv: finv,
            modulo: modulo,
        }
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      mut abs: [(u64, u64); n],
    }
    abs.sort();

    use std::collections::HashMap;
    let mut am = HashMap::new();
    let mut bm = HashMap::new();
    let mut abm = HashMap::new();

    for i in 0..n {
        let a = abs[i].0;
        let b = abs[i].1;
        *am.entry(a).or_insert(0) += 1;
        *bm.entry(b).or_insert(0) += 1;
        *abm.entry(abs[i]).or_insert(0) += 1;
    }

    let fact = ModFactorial::new(n, MOD);

    let mut v1 = 1;
    for (k, &v) in am.iter() {
        v1 *= fact.fact[v];
        v1 %= MOD;
    }
    let mut v2 = 1;
    for (k, &v) in bm.iter() {
        v2 *= fact.fact[v];
        v2 %= MOD;
    }

    // a&b
    let mut ok = true;
    for i in 0..n-1 {
        if !(abs[i].0 <= abs[i+1].0 && abs[i].1 <= abs[i+1].1) {
            ok = false;
            break;
        }
    }
    let mut v3 = 0;
    if ok {
        v3 += 1;
        for (k, &v) in abm.iter() {
            v3 *= fact.fact[v];
            v3 %= MOD;
        }
    }

    debug!(v1, v2, v3);
    let bad = (v1 + v2 + MOD - v3) % MOD;
    let ttl = fact.fact[n];
    let ans = (ttl + MOD - bad) % MOD;
    puts!("{}\n", ans);
}
