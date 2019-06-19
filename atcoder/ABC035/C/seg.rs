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
    use std::cmp::PartialOrd;
    use std::convert::From;
    use std::ops::{Add, AddAssign, Div, Mul};
    pub struct RangeAddSegmentTree<T> {
        n: usize,
        init: T,
        pub data: Vec<T>,
        pub lazy: Vec<T>,
    }
    impl<T> RangeAddSegmentTree<T>
    where
        T: Copy
            + Add<Output = T>
            + AddAssign
            + Div<Output = T>
            + Mul<Output = T>
            + From<u64>
            + PartialOrd,
    {
        pub fn new(size: usize, init: T) -> Self {
            let mut n = 1;
            while n < size {
                n *= 2;
            }
            RangeAddSegmentTree {
                n: n,
                init: init,
                data: vec![init; n * 2 - 1],
                lazy: vec![init; n * 2 - 1],
            }
        }
        pub fn range_add(&mut self, l: usize, r: usize, x: T) {
            let n = self.n;
            self.do_range_add(l, r, x, 0, 0, n);
        }
        fn do_range_add(&mut self, l: usize, r: usize, x: T, k: usize, a: usize, b: usize) {
            self.propagate(k);
            if b <= l || r <= a {
            } else if l <= a && b <= r {
                self.lazy[k] += x * T::from((b - a) as u64);
                self.propagate(k);
            } else {
                self.do_range_add(l, r, x, k * 2 + 1, a, (a + b) / 2);
                self.do_range_add(l, r, x, k * 2 + 2, (a + b) / 2, b);
                self.data[k] = self.data[k * 2 + 1] + self.data[k * 2 + 2];
            }
        }
        fn propagate(&mut self, k: usize) {
            if self.lazy[k] > self.init {
                let two = T::from(2u64);
                if k < self.n - 1 {
                    let value = self.lazy[k] / two;
                    self.lazy[k * 2 + 1] += value;
                    self.lazy[k * 2 + 2] += value;
                }
                self.data[k] += self.lazy[k];
                self.lazy[k] = self.init;
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
                q1 + q2
            }
        }
    }
}


fn main() {
    input!{
      n: usize,
      q: usize,
      lrs: [(usize,usize); q],
    }
    let mut seg = ds::RangeAddSegmentTree::new(n, 0);
    for (l, r) in lrs {
        seg.range_add(l-1, r, 1);
    }
    let mut ans = vec![];
    for i in 0..n {
        let q = seg.query(i, i+1);
        if q%2 == 0 {
            ans.push('0')
        } else {
            ans.push('1')
        }
    }
    println!("{}", ans.into_iter().collect::<String>());
}
