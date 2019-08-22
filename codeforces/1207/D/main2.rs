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
    pub fn new(max_value: u64, modulo: u64) -> Self {
        let mut fact = vec![0; (max_value + 1) as usize];
        let mut inv = vec![0; (max_value + 1) as usize];
        let mut finv = vec![0; (max_value + 1) as usize];
        fact[0] = 1;
        fact[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..max_value + 1 {
            fact[i as usize] = fact[(i - 1) as usize] * i % modulo;
            inv[i as usize] = modulo - inv[ (modulo % i) as usize] * (modulo / i) % modulo;
            finv[i as usize] = finv[(i - 1) as usize] * inv[i as usize] % modulo;
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

    let mut aa: Vec<_> = abs.iter().map(|&v|v.0).collect();
    aa.sort();

    let mut bb: Vec<_> = abs.iter().map(|&v|v.1).collect();
    bb.sort();

    let fact = ModFactorial::new(n as u64, MOD);

    let mut v1 = 1;
    let mut cnt = 1;
    for i in 1..n {
        if aa[i] == aa[i-1] {
            cnt += 1;
        } else {
            v1 *= fact.fact[cnt];
            v1 %= MOD;
            cnt = 1;
        }
    }
    v1 *= fact.fact[cnt];
    v1 %= MOD;

    let mut v2 = 1;
    let mut cnt = 1;
    for i in 1..n {
        if bb[i] == bb[i-1] {
            cnt += 1;
        } else {
            v2 *= fact.fact[cnt];
            v2 %= MOD;
            cnt = 1;
        }
    }
    v2 *= fact.fact[cnt];
    v2 %= MOD;

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
        v3 = 1;
        let mut cnt = 1;
        for i in 1..n {
            if abs[i] == abs[i-1] {
                cnt += 1;
            } else {
                v3 *= fact.fact[cnt];
                v3 %= MOD;
                cnt = 1;
            }
        }
        v3 *= fact.fact[cnt];
        v3 %= MOD;
    }
    debug!(v1, v2, v3);

    let bad = (v1 + v2 + MOD - v3) % MOD;
    let ttl = fact.fact[n];
    let ans = (ttl + MOD - bad) % MOD;
    puts!("{}\n", ans);
}
