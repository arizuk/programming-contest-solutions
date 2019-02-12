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

use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Define custom type to implement a min-heap
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cost(usize);

impl Ord for Cost {
    fn cmp(&self, other: &Cost) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Cost) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

fn main() {
    input!{
      n: usize,
      abcs: [(usize, usize, usize); n-1],
      qq: usize,
      k: usize,
      xys: [(usize, usize); qq],
    }
    let k = k-1;
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abcs {
        g[a-1].push((b-1, c));
        g[b-1].push((a-1, c));
    }

    const INF: usize = 1 << 60;
    let mut ds = vec![INF; n];
    ds[k] = 0;

    let mut q = BinaryHeap::new();
    q.push((Cost(0),k));

    while q.len() > 0 {
        let (ttl_cost, v) = q.pop().unwrap();
        if ds[v] < ttl_cost.0 {
            continue;
        }

        for &(n, nc) in &g[v] {
            if ds[v] + nc < ds[n] {
                ds[n] = ds[v] + nc;
                q.push((Cost(ds[v] + nc), n));
            }
        }
    }

    for i in 0..qq {
        let x = xys[i].0 - 1;
        let y = xys[i].1 - 1;
        println!("{}", ds[x] + ds[y]);
    }
}
