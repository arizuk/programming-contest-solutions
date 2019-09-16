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

#[derive(Debug)]
pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind { par: vec, size: vec![1; n] }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    pub fn unite(&mut self, a: usize, b: usize) {
        let apar = self.find(a);
        let bpar = self.find(b);
        if apar != bpar {
            self.size[bpar] += self.size[apar];
        }
        self.par[apar] = bpar;
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
      t: String,
    }
    let mut ss = s.clone();
    ss += s.as_str();
    while ss.len() < t.len() * 2 {
        ss += s.as_str();
    }

    let temp: String = t.clone() + ss.as_str();
    let (rh1, rh2) = RollingHash::create_recommended_pair(temp.as_bytes());

    let n = s.len();
    let mut ok = vec![false; n];
    for i in 0..n {
        let h1 = (
            rh1.get(0, t.len()), rh2.get(0, t.len())
        );
        let h2 = (
            rh1.get(t.len()+i, t.len()*2+i),
            rh2.get(t.len()+i, t.len()*2+i)
        );
        if h1 == h2 {
            ok[i] = true;
        }
    }

    let n = s.len();
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        if ok[i] {
            let nx = (i+t.len())%n;
            if uf.same(i, nx) {
                return puts!("{}\n", -1);
            }
            uf.unite(i, nx);
        }
    }

    let mut ans = 1;
    for i in 0..n {
        ans = max(ans, uf.size[i]);
    }
    puts!("{}\n", ans-1);
}
