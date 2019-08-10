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
use std::collections::BinaryHeap;
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

const INF: usize = 1 << 50;

fn dijkstra(edges: &Vec<Vec<(usize,usize)>>, n:usize,s:usize,t:usize) -> Vec<usize> {
    let mut q = BinaryHeap::new();
    q.push((Rev(0),s));
    let mut dist = vec![INF; n];

    while q.len() > 0 {
        let (Rev(cur_dist), node) = q.pop().unwrap();
        if dist[node] <= cur_dist {
            continue;
        }
        dist[node] = cur_dist;
        for &(nx_node, nx_dist) in edges[node].iter() {
            if cur_dist + nx_dist < dist[nx_node] {
                q.push(
                    (Rev(cur_dist + nx_dist), (nx_node))
                );
            }
        }
    }
    dist
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      s: usize1,
      t: usize1,
      uvabs: [(usize1,usize1,usize,usize); m]
    }

    let mut yen_es = vec![vec![]; n];
    let mut snuke_ens = vec![vec![]; n];
    for &(u, v, a, b) in uvabs.iter() {
        yen_es[u].push((v,a));
        yen_es[v].push((u,a));

        snuke_ens[u].push((v,b));
        snuke_ens[v].push((u,b));
    }

    let dist1 = dijkstra(&yen_es, n, s, t);
    let dist2 = dijkstra(&snuke_ens, n, t, s);
    let dist: Vec<_> = (0..n).map(|i| dist1[i] + dist2[i]).collect();


    let mut heap = BinaryHeap::new();
    let mut ans = vec![];
    for i in (0..n).rev() {
        heap.push( Rev(dist[i]) );
        let Rev(peek) = *heap.peek().unwrap();
        ans.push(peek);
    }
    ans.reverse();

    let ttl = 1e15 as usize;
    for a in ans {
        puts!("{}\n", ttl-a);
    }
}
