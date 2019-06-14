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
use std::cmp::Ordering;

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
    input!{
      n: usize,
      m: usize,
      pqcs: [(usize,usize,usize); m],
    }
    let mut nodes = vec![];
    for i in 0..n {
        nodes.push((i+1, 0));
    }
    for &(p, q, c) in pqcs.iter() {
        nodes.push((p, c));
        nodes.push((q, c));
    }
    nodes.sort();
    nodes.dedup();

    let nn = nodes.len();
    let mut edges = vec![vec![]; nn];

    for i in 0..nn {
        let node = nodes[i];
        if node.1 == 0 { continue; }
        let out = nodes.binary_search(&(node.0, 0)).unwrap();

        edges[i].push((out, 0)); // 降りるとき
        edges[out].push((i, 1)); // 乗るとき
    }

    for &(p, q, c) in pqcs.iter() {
        // p.c -> q.c 0
        // q.c -> p.c 0
        let pc = nodes.binary_search(&(p, c)).unwrap();
        let qc = nodes.binary_search(&(q, c)).unwrap();
        edges[pc].push((qc, 0));
        edges[qc].push((pc, 0));
    }

    let s = nodes.binary_search(&(1, 0)).unwrap();
    let e = nodes.binary_search(&(n, 0)).unwrap();

    // 01DFS
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    const INF: usize = 1 << 31;
    let mut dist = vec![INF; nn];
    dist[s] = 0;
    q.push_back(s);
    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        for &(next_node, cost) in edges[cur].iter() {
            if dist[cur] + cost < dist[next_node] {
                dist[next_node] = dist[cur] + cost;
                if cost == 1 {
                    q.push_back(next_node);
                } else {
                    q.push_front(next_node);
                }
            }
        }
    }
    if dist[e] >= INF {
        println!("{}", -1);
    } else {
        println!("{}", dist[e]);
    }
}
