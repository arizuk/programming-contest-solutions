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

type Pair = (usize,usize);

fn rec<F>(seg: &SegmentTree<Pair, F>, l:usize, r:usize) -> usize
where F: Fn(Pair, Pair) -> Pair
{
    if l >= r {
        return 0;
    }

    let (v, idx) = seg.query(l, r);
    let mut ret = (idx-l+1) * v * (r-idx);
    ret += rec(seg, l, idx);
    ret += rec(seg, idx+1, r);
    ret
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      aa: [usize; n],
    }

    let mut seg = SegmentTree::new(n, (n,n), |a,b| if a < b {a} else {b});
    for i in 0..n {
        seg.set(i, (aa[i], i));
    }
    let ans = rec(&seg, 0, n);
    puts!("{}\n", ans);
}
