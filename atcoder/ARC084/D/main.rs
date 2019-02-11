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
      k: usize,
    }

    let mut g = vec![vec![]; k];
    for i in 1..k {
        let n = (i+1)%k;
        g[i].push( (n, 1) );

        let n = (i*10)%k;
        if i != n {
            g[i].push( (n, 0) );
        }
    }

    const INF: usize = 1 << 32;
    let mut ds = vec![INF; k];
    ds[1] = 1;

    let mut q = BinaryHeap::new();
    q.push((Cost(1),1));

    while q.len() > 0 {
        // debug!(q);
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
    println!("{}", ds[0]);
}
