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
    input!{
      n: u64,
      mut xys: [(u64, u64); n],
    }
    xys.sort();

    let mut x_par = vec![None; 100000+1];
    let mut y_par = vec![None; 100000+1];


    let mut uf = UnionFind::new(n as usize);
    for i in 0..n as usize {
        let &(x, y) = &xys[i];

        if let Some(p) = x_par[x as usize] {
            uf.unite(i, p);
        } else {
            x_par[x as usize] = Some(i);
        }

        if let Some(p) = y_par[y as usize] {
            uf.unite(i, p);
        } else {
            y_par[y as usize] = Some(i);
        }
    }

    let mut pairs = vec![vec![]; n as usize];
    for i in 0..n as usize {
        let par = uf.find(i);
        pairs[par].push(xys[i].clone());
    }

    let mut ans = 0;
    for i in 0..n as usize {
        let pairs = pairs[i].clone();
        let mut xs: Vec<_> = pairs.iter().map(|v| v.0).collect();
        let mut ys: Vec<_> = pairs.iter().map(|v| v.1).collect();
        xs.sort();
        xs.dedup();
        ys.sort();
        ys.dedup();
        ans += xs.len() as u64 * ys.len() as u64;
    }
    println!("{}", ans-n);
}
