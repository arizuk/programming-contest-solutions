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
    use std::collections::VecDeque;
    pub struct SlidingWindowQ<F> {
        n: usize,
        window: usize,
        q: VecDeque<usize>,
        cur: usize,
        f: F,
    }
    impl<F> SlidingWindowQ<F>
    where
        F: Fn(usize, usize) -> bool,
    {
        #[doc = " ```ignore"]
        #[doc = " let mut q = SlidingWindowQ::new(3, 2, |a, b| f(a) <= f(b));"]
        #[doc = " ```"]
        pub fn new(n: usize, window: usize, f: F) -> Self {
            SlidingWindowQ {
                n: n,
                q: VecDeque::new(),
                window: window,
                f: f,
                cur: 0,
            }
        }
        pub fn next(&mut self) -> Option<usize> {
            let i = self.cur;
            if i == self.n {
                return None;
            }
            while self.q.len() > 0 {
                let j = *self.q.back().unwrap();
                if (self.f)(i, j) {
                    self.q.pop_back();
                } else {
                    break;
                }
            }
            self.q.push_back(i);
            let j = *self.q.front().unwrap();
            if i >= self.window && j == i - self.window {
                self.q.pop_front();
            }
            self.cur += 1;
            self.front()
        }
        pub fn front(&self) -> Option<usize> {
            self.q.front().map(|&v| v)
        }
    }
}
use ds::SlidingWindowQ;

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      k: usize,
      ps: [usize; n],
    }

    let mut uf = UnionFind::new(n-k+1);
    let mut start = 0;
    let mut incr = None;
    for i in 1..n {
        if ps[i-1] > ps[i] {
            start = i;
        } else {
            if i-start+1 >= k {
                if let Some(idx) = incr {
                    uf.unite(idx, start);
                } else {
                    incr = Some(start);
                }
            }
        }
    }

    let mut sw1 = SlidingWindowQ::new(n, k+1, |a, b| ps[a] < ps[b]);
    let mut sw2 = SlidingWindowQ::new(n, k+1, |a, b| ps[a] > ps[b]);

    for _ in 0..k {
        sw1.next();
        sw2.next();
    }

    for i in k..n {
        let min_v = ps[ sw1.next().unwrap() ];
        let max_v = ps[ sw2.next().unwrap() ];

        // Same to the prev range
        if min_v == ps[i-k] && max_v == ps[i] {
            uf.unite(i-k, i-k+1);
        }
    }

    let mut pars = vec![];
    for i in 0..n-k+1 {
        pars.push(uf.find(i));
    }
    pars.sort();
    pars.dedup();
    puts!("{}\n", pars.len());
}
