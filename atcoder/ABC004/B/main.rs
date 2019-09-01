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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      x: usize,
      aa: [usize; n],
    }

    use ds::SlidingWindowQ;
    let mut ans = 1 << 60;
    for k in 0..n {
        let mut q = SlidingWindowQ::new(n*2, k+1, |i, j| aa[i%n] <= aa[j%n]);
        for _ in 0..n {
            q.next();
        }
        let mut temp = k * x;
        for i in 0..n {
            let idx = q.next().unwrap();
            temp += aa[ idx%n ];
        }
        ans = min(ans, temp);
    }
    puts!("{}\n", ans);
}
