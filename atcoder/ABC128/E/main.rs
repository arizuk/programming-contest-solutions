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

pub mod ds {
    pub struct LazySegmentTree<T, F> {
        n: usize,
        init: T,
        f: F,
        pub data: Vec<T>,
        pub lazy: Vec<Option<T>>,
    }
    impl<T, F> LazySegmentTree<T, F>
    where
        T: Copy,
        F: Fn(T, T) -> T,
    {
        pub fn new(size: usize, init: T, f: F) -> Self {
            let mut n = 1;
            while n < size {
                n *= 2;
            }
            LazySegmentTree {
                n: n,
                init: init,
                f: f,
                data: vec![init; n * 2 - 1],
                lazy: vec![None; n * 2 - 1],
            }
        }
        pub fn range_update(&mut self, l: usize, r: usize, x: T) {
            let n = self.n;
            self.do_range_update(l, r, x, 0, 0, n);
        }
        fn do_range_update(&mut self, l: usize, r: usize, x: T, k: usize, a: usize, b: usize) {
            self.propagate(k);
            if b <= l || r <= a {
            } else if l <= a && b <= r {
                self.lazy[k] = Some(x);
                self.propagate(k);
            } else {
                self.do_range_update(l, r, x, k * 2 + 1, a, (a + b) / 2);
                self.do_range_update(l, r, x, k * 2 + 2, (a + b) / 2, b);
                self.data[k] = (self.f)(self.data[k * 2 + 1], self.data[k * 2 + 2]);
            }
        }
        fn propagate(&mut self, k: usize) {
            if let Some(x) = self.lazy[k] {
                if k < self.n - 1 {
                    if let Some(y) = self.lazy[k * 2 + 1] {
                        self.lazy[k * 2 + 1] = Some((self.f)(x, y));
                    } else {
                        self.lazy[k * 2 + 1] = Some(x);
                    }

                    if let Some(y) = self.lazy[k * 2 + 2] {
                        self.lazy[k * 2 + 2] = Some((self.f)(x, y));
                    } else {
                        self.lazy[k * 2 + 2] = Some(x);
                    }
                }
                self.data[k] = (self.f)(self.data[k], x);
                self.lazy[k] = None;
            }
        }
        #[doc = " [l, r)"]
        pub fn query(&mut self, l: usize, r: usize) -> T {
            assert!(l < r);
            let n = self.n;
            self.do_query(l, r, 0, 0, n)
        }
        fn do_query(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
            self.propagate(k);
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
    input!{
      n: usize,
      q: usize,
      stxs: [(i64,i64,i64);n],
      ds: [i64; q]
    }
    use ds::LazySegmentTree;

    let mut ts = vec![];
    for &(s, t, x) in stxs.iter() {
        let l = s-x;
        let r = t-x;
        ts.push(l);
        ts.push(r);
    }
    for &d in ds.iter() {
        ts.push(d);
    }
    ts.sort();
    ts.dedup();

    const MAX: i64 = 1 << 60;
    let mut seg = LazySegmentTree::new(ts.len(), MAX, |a,b| min(a,b));
    for &(s, t, x) in stxs.iter() {
        let l = ts.binary_search(&(s-x)).unwrap();
        let r = ts.binary_search(&(t-x)).unwrap();
        seg.range_update(l, r, x);
    }
    // debug!(ts);
    // debug!(seg.data, seg.lazy);

    for d in ds {
        let l = ts.binary_search(&d).unwrap();
        let x = seg.query(l, l+1);
        if x == MAX {
            println!("{}", -1);
        } else {
            println!("{}", x);
        }
    }
}
