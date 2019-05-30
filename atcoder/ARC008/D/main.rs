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
        pub fn alter(&mut self, mut k: usize, x: T) {
            k += self.n - 1;
            self.data[k] = (self.f)(self.data[k], x);
            while k > 0 {
                k = (k - 1) / 2;
                self.data[k] = (self.f)(self.data[2 * k + 1], self.data[2 * k + 2]);
            }
        }
        pub fn update(&mut self, mut k: usize, x: T) {
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
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };
    let n:usize = sc.read();
    let m:usize = sc.read();

    let mut ps = vec![];
    let mut pabs = vec![];
    for i in 0..m {
        let p: usize = sc.read::<usize>() - 1;
        let a: f64 = sc.read();
        let b: f64 = sc.read();
        ps.push(p);
        pabs.push((p, a, b));
    }
    ps.sort();
    ps.dedup();
    let mut map = std::collections::HashMap::new();
    for i in 0..ps.len() {
        map.insert(ps[i], i);
    }

    use ds::SegmentTree;
    let f = |(a1,b1), (a2,b2)| {
        (a1 * a2, b1 * a2  + b2)
    };
    let mut seg = SegmentTree::new(ps.len(), (1.0, 0.0), f);

    let mut min_value = 1.0;
    let mut max_value = 1.0;

    for (p, a, b) in pabs {
        let i = map[&p];
        seg.update(i, (a,b));
        let (a, b) = seg.query(0, n);
        let value = a + b;
        if value < min_value {
            min_value = value;
        }
        if value > max_value {
            max_value = value;
        }
    }
    println!("{}", min_value);
    println!("{}", max_value);
}

pub struct Scanner<R> {
    reader: R,
}
impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf: String = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect();
        buf.parse::<T>().ok().expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}