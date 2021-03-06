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


fn main() {
    input!{
      n: usize,
      m: usize,
      pqcs: [(usize1,usize1,usize1); m],
    }
    let mut nodes = vec![];
    for &(p, q, c) in pqcs.iter() {
        nodes.push((p, c));
        nodes.push((q, c));
    }
    nodes.sort();
    nodes.dedup();

    let nn = nodes.len();
    let mut uf = UnionFind::new(nn);
    for &(p, q, c) in pqcs.iter() {
        let pi = nodes.binary_search(&(p, c)).unwrap();
        let qi = nodes.binary_search(&(q, c)).unwrap();
        uf.unite(pi, qi);
    }

    let mut edges = vec![vec![]; n+nn];
    for &(p, q, c) in pqcs.iter() {
        let pi = nodes.binary_search(&(p, c)).unwrap();
        let par = uf.find(pi) + n;

        edges[p].push(par);
        edges[par].push(p);
        edges[q].push(par);
        edges[par].push(q);
    }

    // 01DFS
    let s = 0;
    let e = n-1;
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    const INF: usize = 1 << 31;
    let mut dist = vec![INF; n+nn];
    dist[s] = 0;
    q.push_back(s);
    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        for &next_node in edges[cur].iter() {
            if dist[cur] + 1 < dist[next_node] {
                dist[next_node] = dist[cur] + 1;
                q.push_back(next_node);
            }
        }
    }
    if dist[e] >= INF {
        println!("{}", -1);
    } else {
        println!("{}", dist[e]/2);
    }
}
