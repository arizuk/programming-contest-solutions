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

#[derive(Debug)]
pub struct UnionFind {
    par: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind { par: vec }
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
      mut a: usize,
      mut b: usize,
      mut c: usize,
      mut d: usize,
      n: usize,
      xys: [(usize, usize); n]
    }
    if a == 0 { a = 1e10 as usize; }
    if b == 0 { b = 1e10 as usize; }
    if c == 0 { c = 1e10 as usize; }
    if d == 0 { d = 1e10 as usize; }

    use std::collections::HashMap;
    let mut mod1 = HashMap::new();
    let mut mod2 = HashMap::new();

    for i in 0..n {
        let (x, y) = xys[i];
        mod1.entry((x%a, y%b)).or_insert(vec![]).push(i);
        mod2.entry((x%c, y%d)).or_insert(vec![]).push(i);
    }

    debug!(mod1);
    debug!(mod2);

    let mut uf = UnionFind::new(n);
    for (k, v) in mod1 {
        let v0 = v[0];
        for idx in v { uf.unite(v0, idx); }
    }
    for (k, v) in mod2 {
        let v0 = v[0];
        for idx in v { uf.unite(v0, idx); }
    }

    let mut pairs = vec![];
    for i in 0..n {
        pairs.push(uf.find(i));
    }
    debug!(pairs);
    pairs.sort();
    pairs.dedup();
    puts!("{}\n", pairs.len());
}
