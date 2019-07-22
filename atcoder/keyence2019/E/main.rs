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

pub mod ds {
    pub struct SegmentTree<T, F> {
        n: usize,
        pub data: Vec<T>,
        init: T,
        f: F,
    }
    impl<T, F> SegmentTree<T, F>
    where
        T: Copy,
        F: Fn(T, T) -> T,
    {
        pub fn new(size: usize, init: T, f: F) -> Self {
            let mut n = 1;
            while n < size {
                n *= 2;
            }
            SegmentTree {
                n: n,
                init: init,
                f: f,
                data: vec![init; n * 2 - 1],
            }
        }
        pub fn update(&mut self, mut k: usize, x: T) {
            k += self.n - 1;
            self.data[k] = (self.f)(self.data[k], x);
            while k > 0 {
                k = (k - 1) / 2;
                self.data[k] = (self.f)(self.data[2 * k + 1], self.data[2 * k + 2]);
            }
        }
        pub fn set(&mut self, mut k: usize, x: T) {
            k += self.n - 1;
            self.data[k] = x;
            while k > 0 {
                k = (k - 1) / 2;
                self.data[k] = (self.f)(self.data[2 * k + 1], self.data[2 * k + 2]);
            }
        }
        #[doc = " [l, r)"]
        pub fn query(&self, l: usize, r: usize) -> T {
            assert!(l < r);
            self.do_query(l, r, 0, 0, self.n)
        }
        fn do_query(&self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
            if b <= l || r <= a {
                self.init
            } else if l <= a && b <= r {
                self.data[k]
            } else {
                let q1 = self.do_query(l, r, k * 2 + 1, a, (a + b) / 2);
                let q2 = self.do_query(l, r, k * 2 + 2, (a + b) / 2, b);
                (self.f)(q1, q2)
            }
        }
    }
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    use ds::SegmentTree;

    input!{
      n: usize,
      d: usize,
      aa: [usize; n],
    }
    const INF: usize = 1 << 50;

    let mut seg_left = SegmentTree::new(n, (INF,0), |a, b| if a.0 < b.0 { a } else { b });
    let mut seg_right = SegmentTree::new(n, (INF,0), |a, b| if a.0 < b.0 { a } else { b });

    for i in 0..n {
        seg_right.set(i, (i*d + aa[i], i));
        let c = d * (n - i - 1);
        seg_left.set(i, (c + aa[i], i));
    }

    let mut bb: Vec<_> = aa.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    bb.sort_by_key(|v| v.1);
    bb.reverse();

    let mut edges = vec![];
    for i in 0..n {
        let (idx, _) = bb[i];
        if idx < n - 1 {
            let (_, j) = seg_right.query(idx+1, n);
            let cost = ((idx as i64) - (j as i64)).abs() as usize * d + aa[idx] + aa[j];
            edges.push((cost,idx,j));
            seg_right.set(idx, (INF, idx));
        }
        if idx > 0 {
            let (_, j) = seg_left.query(0, idx);
            let cost = ((idx as i64) - (j as i64)).abs() as usize * d + aa[idx] + aa[j];
            edges.push((cost,idx,j));
            seg_left.set(idx, (INF, idx));
        }
    }
    edges.sort();

    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for &(c, i, j) in edges.iter() {
        if !uf.same(i, j) {
            uf.unite(i, j);
            ans += c;
        }
    }
    puts!("{}\n", ans);
}
