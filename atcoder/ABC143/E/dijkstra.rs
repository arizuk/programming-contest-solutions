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

pub mod dijkstra {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    type Dist = i64;
    pub const INF: Dist = 1 << 60;
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct Rev(Dist);

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
    pub fn shortest_path(edges: &Vec<Vec<(usize, Dist)>>, s: usize, l: Dist) -> Vec<(Dist, Dist)> {
        let n = edges.len();
        let mut dist = vec![(INF, 0); n];
        dist[s] = (0, l);

        let mut heap = BinaryHeap::new();
        heap.push((Rev(0), l, s));


        while let Some((Rev(cur_times), cur_gas, cur)) = heap.pop() {

            if dist[cur].0 < cur_times {
                continue;
            }
            if dist[cur].0 == cur_times && cur_gas < dist[cur].1 {
                continue;
            }

            for &(adj, cost) in edges[cur].iter() {
                let nx_times;
                let nx_gas;

                if cur_gas >= cost {
                    nx_times = cur_times;
                    nx_gas = cur_gas - cost;
                } else {
                    nx_times = cur_times+1;
                    nx_gas = l - cost;
                }

                if dist[adj].0 < nx_times {
                    continue;
                }
                if dist[adj].0 == nx_times && nx_gas < dist[adj].1 {
                    continue;
                }

                dist[adj] = (nx_times, nx_gas);
                heap.push((Rev(nx_times), nx_gas, adj));
            }
        }
        dist
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};
#[allow(unused_imports)]
use std::io::{stdout, stdin, BufWriter, Write};

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      l: i64,
      abcs: [(usize1, usize1, i64); m],
      q: usize,
      sts: [(usize1, usize1); q],
    }

    let mut edges = vec![vec![]; n];
    for &(a, b, c) in abcs.iter() {
        if c <= l {
            edges[a].push((b,c));
            edges[b].push((a,c));
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        let dist = dijkstra::shortest_path(&edges, i, l);
        ans.push(dist);
    }

    for (s, t) in sts {
        let times = ans[s][t].0;
        if times >= dijkstra::INF {
            puts!("{}\n", -1);
        } else {
            puts!("{}\n", times);
        }
    }
}
