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

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rev(usize);

impl Ord for Rev {
    fn cmp(&self, other: &Rev) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Rev {
    fn partial_cmp(&self, other: &Rev) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

#[derive(Copy, Clone, Ord, Eq, PartialEq,PartialOrd)]
struct Range {
    l: usize,
    r: usize,
    ml: usize,
    mr: usize,
}
impl Range {
    fn new(l:usize, r:usize, ml:usize, mr:usize) ->Self {
        Range {
            l: l,
            r: r,
            ml: ml,
            mr: mr,
        }
    }
}
use std::fmt;
impl fmt::Debug for Range {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        write!(f, "[{} {} {} {})", self.l, self.ml, self.mr, self.r)
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
use ds::SegmentTree;
type Pair = (usize,usize);

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      ps: [usize; n],
    }

    let f0 = |a: Pair, b: Pair| if a.0 < b.0 { a } else { b };
    let f1 = |a: Pair, b: Pair| if a.0 < b.0 { a } else { b };
    let mut seg0 = SegmentTree::new(n, (n+1,n), f0);
    let mut seg1 = SegmentTree::new(n, (n+1,n), f1);

    for i in 0..n {
        if i%2==0 {
            seg0.set(i, (ps[i], i));
        } else {
            seg1.set(i, (ps[i], i));
        }
    }

    let get_sub_range = |l, r| {
        if l%2==0 {
            let (_, ll) = seg0.query(l, r);
            let (_, rr) = seg1.query(ll+1, r);
            Range::new(l, r, ll, rr+1)
        } else {
            let (_, ll) = seg1.query(l, r);
            let (_, rr) = seg0.query(ll+1, r);
            Range::new(l, r, ll, rr+1)
        }
    };


    let range = get_sub_range(0, n);
    let mut heap = BinaryHeap::new();
    heap.push((
        Rev( ps[range.ml] ), range
    ));

    let mut seen = vec![false; n];

    let mut ans = vec![];
    while heap.len() >0 {
        let (_, range) = heap.pop().unwrap();
        ans.push(range.ml);
        ans.push(range.mr-1);

        assert!(seen[range.ml] == false);
        seen[range.ml] = true;

        let rs = [
            (range.l, range.ml),
            (range.ml+1, range.mr-1),
            (range.mr, range.r),
        ];
        for &(l, r) in rs.iter() {
            if l < r {
                let new_range = get_sub_range(l, r);
                heap.push((
                    Rev( ps[new_range.ml] ), new_range
                ));
            }
        }
    }
    for &a in ans.iter() {
        if a == ans[ans.len()-1] {
            puts!("{}", ps[a]);
        } else {
            puts!("{} ", ps[a]);
        }
    }
    puts!("{}\n", "");

}
