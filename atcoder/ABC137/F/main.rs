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

fn mod_pow(mut cur: usize, mut e: usize, p: usize) -> usize {
    let mut result = 1;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * cur) % p;
        }
        cur = (cur * cur) % p;
        e >>= 1;
    }
    result
}

#[allow(dead_code)]
pub struct ModFactorial {
    fact: Vec<usize>,
    inv: Vec<usize>,
    finv: Vec<usize>,
    modulo: usize,
}
impl ModFactorial {
    pub fn new(max_value: usize, modulo: usize) -> Self {
        let mut fact = vec![0; max_value + 1];
        let mut inv = vec![0; max_value + 1];
        let mut finv = vec![0; max_value + 1];
        fact[0] = 1;
        fact[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..max_value + 1 {
            fact[i] = fact[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }
        ModFactorial {
            fact: fact,
            inv: inv,
            finv: finv,
            modulo: modulo,
        }
    }
    pub fn combination(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        self.fact[n] * self.finv[n - k] % self.modulo * self.finv[k] % self.modulo
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      p: usize,
      aa: [usize; p],
    }

    let mut bs = vec![0; p];
    let n = p-1;
    let fact = ModFactorial::new(n, p);
    for i in 0..p {
        if aa[i] == 1 {
            bs[0] += 1;
            bs[0] %= p;

            for j in 0..n+1 {
                let idx = n-j;
                let temp = fact.combination(n, j) * mod_pow(i, j, p) % p;
                if j % 2 == 0 {
                    bs[idx] += p - temp;
                } else {
                    bs[idx] += temp;
                }
                bs[idx] %= p;
            }
        }
    }
    for i in 0..bs.len() {
        if i == bs.len() - 1 {
            puts!("{}", bs[i]);
        } else {
            puts!("{} ", bs[i]);
        }
    }
    puts!("{}\n", "");
}
