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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      xys: [(usize,usize); n],
    }

    let mut xs: Vec<_> = xys.iter().cloned().enumerate().collect();
    xs.sort_by_key(|v| (v.1).0);
    let mut ys: Vec<_> = xys.iter().cloned().enumerate().collect();
    ys.sort_by_key(|v| (v.1).1);

    // Prim's algorithm
    let mut edges = vec![vec![]; n];
    for i in 0..n-1 {
        let (i1, (x1, _)) = xs[i];
        let (i2, (x2, _)) = xs[i+1];

        edges[i1].push((i2, x2-x1));
        edges[i2].push((i1, x2-x1));

        let (i1, (_, y1)) = ys[i];
        let (i2, (_, y2)) = ys[i+1];
        edges[i1].push((i2, y2-y1));
        edges[i2].push((i1, y2-y1));
    }

    let mut q = BinaryHeap::new();
    let mut used = vec![false; n];
    q.push((Rev(0), 0));

    let mut ans = 0;
    while let Some((Rev(cost), cur)) = q.pop() {
        if used[cur] {
            continue;
        }
        used[cur] = true;
        ans += cost;
        for &(nx, nx_cost) in edges[cur].iter() {
            if !used[nx] {
                q.push((Rev(nx_cost), nx));
            }
        }
    }
    puts!("{}\n", ans);
}
