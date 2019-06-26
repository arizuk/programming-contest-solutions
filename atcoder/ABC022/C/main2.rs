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
use std::cmp::Ordering;

#[allow(unused_imports)]
use std::io::Write;

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
      uvls: [(usize1,usize1,usize); m]
    }

    let mut edges = vec![vec![]; n*2];
    let mut i = 0;
    for &(u, v, l) in uvls.iter() {
        if u == 0 {
            edges[n+i].push((v, l));
            edges[v].push((n+i, l));
            i += 1;
        } else {
            edges[u].push((v,l));
            edges[v].push((u,l));
        }
    }
    use std::collections::BinaryHeap;

    // ダイクストラ
    // Dijkstra's algorithm
    const INF: usize = 1 << 60;

    let mut ans = INF;
    for j in 0..i {
        let s = n+j;

        let mut dist = vec![INF; n+i];
        let mut q = BinaryHeap::new();
        q.push((Rev(0),s));
        dist[s] = 0;

        while q.len() > 0 {
            let (Rev(d), cur) = q.pop().unwrap();
            if dist[cur] < d {
                continue;
            }

            for &(nd, l) in edges[cur].iter() {
                if dist[nd] > dist[cur] + l {
                    dist[nd] = dist[cur] + l;
                    q.push((Rev(dist[nd]), nd));
                }
            }
        }

        for k in 0..i {
            if j == k { continue; }
            ans = min(ans, dist[n+k]);
        }
    }
    if ans >= INF {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
