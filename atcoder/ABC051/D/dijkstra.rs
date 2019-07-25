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


#[allow(unused_imports)]
use std::cmp::{min, max};
#[allow(unused_imports)]
use std::io::{stdout, stdin, BufWriter, Write};
use std::collections::HashSet;

fn dfs(edges: &Vec<Vec<(usize,usize)>>, cur: usize, par: usize, d: usize, used: &mut HashSet<(usize,usize)>, dist: &Vec<usize>) {
    for &(nx, c) in edges[cur].iter() {
        if cur == par { continue }
        if d + c == dist[nx] {
            let pair = if cur < nx { (cur, nx) } else { (nx, cur) };
            used.insert(pair);
            dfs(edges, nx, cur, d+c, used, dist);
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
      m: usize,
      abcs: [(usize1,usize1,usize); m],
    }

    let mut edges = vec![vec![]; n];
    for &(a, b, c) in abcs.iter() {
        edges[a].push((b,c));
        edges[b].push((a,c));
    }

    let mut used = HashSet::new();
    // ダイクストラ
    // Dijkstra's algorithm

    const INF: usize = 1 << 50;
    for i in 0..n {
        use std::collections::BinaryHeap;
        let mut q = BinaryHeap::new();
        q.push((Rev(0),i));
        let mut dist = vec![INF; n];

        while q.len() > 0 {
            let (Rev(cur_dist), node) = q.pop().unwrap();
            if dist[node] < cur_dist {
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
        dfs(&edges, i, n, 0, &mut used, &dist);
    }

    let mut ans = 0;
    for (a, b, _) in abcs {
        let pair = if a < b { (a, b) } else { (b, a) };
        if !used.contains(&pair) {
            ans += 1;
        }
    }
    puts!("{}\n", ans);
}
