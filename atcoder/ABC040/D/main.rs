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
    pub size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        let size = vec![1; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind { par: vec, size: size }
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
        if apar != bpar {
            self.size[bpar] += self.size[apar];
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
      m: usize,
      abys: [(usize1, usize1, usize); m],
      q: usize,
      vws: [(usize1, usize); q]
    }

    let mut uf = UnionFind::new(n);
    let mut bridges = vec![vec![]; 2e5 as usize + 2];
    let mut persons = vec![vec![]; 2e5 as usize + 2];
    for (a, b, y) in abys {
        bridges[y].push((a,b));
    }

    for i in 0..q {
        let (v, w) = vws[i];
        persons[w].push((i, v));
    }

    let mut ans = vec![0; q];
    for w in (0..2e5 as usize +1).rev() {
        for &(i, v) in persons[w].iter() {
            let p = uf.find(v);
            ans[i] = uf.size[p];
        }

        for &(a,b) in bridges[w].iter() {
            uf.unite(a, b);
        }
    }

    for a in ans {
        puts!("{}\n", a);
    }
}
