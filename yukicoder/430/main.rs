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

struct RollingHash2 {
    f1: RollingHash,
    f2: RollingHash,
}

impl RollingHash2 {
    fn new(s: &[u8]) -> Self {
        let (f1, f2) = RollingHash::create_recommended_pair(s);
        RollingHash2 { f1: f1, f2: f2 }
    }

    fn get(&self, l:usize, r:usize) -> (u64,u64) {
        (self.f1.get(l, r), self.f2.get(l, r))
    }
}

pub struct RollingHash {
    modulo: u64,
    power: Vec<u64>,
    hash: Vec<u64>,
}
impl RollingHash {
    pub fn create_recommended_pair(s: &[u8]) -> (Self, Self) {
        (
            RollingHash::new(s, 1009, 1000000007),
            RollingHash::new(s, 1007, 1000000009),
        )
    }
    pub fn new(s: &[u8], base: u64, modulo: u64) -> Self {
        let n = s.len();
        let mut power = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        for i in 0..n {
            power[i + 1] = power[i] * base % modulo;
            hash[i + 1] = (hash[i] * base + s[i] as u64) % modulo;
        }
        RollingHash {
            modulo: modulo,
            power: power,
            hash: hash,
        }
    }
    #[doc = " [l, r)"]
    pub fn get(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + self.modulo - (self.hash[l] * self.power[r - l]) % self.modulo)
            % self.modulo
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      s: String,
      m: usize,
      cs: [String; m],
    }
    let rh = RollingHash2::new(s.as_bytes());
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let n = s.len();
    for len in 1..min(n, 10)+1 {
        for i in 0..n-len+1 {
            let h = rh.get(i, i+len);
            *map.entry(h).or_insert(0) += 1;
        }
    }
    let mut ans = 0;
    for c in cs {
        let rh = RollingHash2::new(c.as_bytes());
        let h = rh.get(0, c.len());
        if map.contains_key(&h) {
            ans += map[&h];
            // debug!(c, map[&h]);
        }
    }
    puts!("{}\n", ans);
}
