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
use ds::SegmentTree;

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let n:usize = sc.read();
    let q:usize = sc.read();
    let aa: Vec<i64> = sc.read_vec(n);
    let mut seg1 = SegmentTree::new(n, 0i64, |a, b| a+b);
    let mut seg2 = SegmentTree::new(n, 0i64, |a, b| a+b);

    for i in 0..n-1 {
        let v = if aa[i] == aa[i+1] { 0 } else { 1 };
        seg2.set(i, v);
    }

    for _ in 0..q {
        let t:usize = sc.read();
        let l:usize = sc.read::<usize>()-1;
        let r:usize = sc.read::<usize>()-1;
        if t == 1 {
            let x:i64 = sc.read::<i64>();
            seg1.update(l, x);
            if r+1 < n {
                seg1.update(r+1, -1 * x);
            }
            // l-1,l
            if l > 0 {
                let v1 = seg1.query(0, l);
                let v2 = seg1.query(0, l+1);
                if aa[l-1]+v1 == aa[l]+v2 {
                    seg2.set(l-1, 0);
                } else {
                    seg2.set(l-1, 1);
                }
            }

            // r,r+1
            if r < n-1 {
                let v1 = seg1.query(0, r+1);
                let v2 = seg1.query(0, r+2);
                if aa[r]+v1 == aa[r+1]+v2 {
                    seg2.set(r, 0);
                } else {
                    seg2.set(r, 1);
                }
            }
        } else {
            let sum = if l == r {
                0
            } else {
                seg2.query(l, r)
            };
            puts!("{}\n", sum+1);
        }
    }
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