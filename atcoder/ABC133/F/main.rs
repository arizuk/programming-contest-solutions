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

use std::collections::HashSet;
use std::collections::HashMap;


type Pair = (usize,usize,usize);

fn dfs1(
    edges: &Vec<Vec<Pair>>, cur: usize, par: usize,
    pars: &mut Vec<Vec<usize>>, depths: &mut Vec<usize>, dist: &mut Vec<usize>,
    d: usize
) {
    pars[0][cur] = par;
    depths[cur] = d;

    for &(nd, _, n_dist) in edges[cur].iter() {
        if nd != par {
            dist[nd] = dist[cur] + n_dist;
            dfs1(edges, nd, cur, pars ,depths, dist, d+1);
        }
    }
}

fn dfs2(
    edges: &Vec<Vec<Pair>>, cur: usize, par: usize,
    cdist: &mut Vec<usize>, cnums: &mut Vec<usize>,
    qs: &Vec<HashSet<usize>>,
    mdist: &mut Vec<HashMap<usize, usize>>, mnums: &mut Vec<HashMap<usize, usize>>,
) {
    for &query_color in qs[cur].iter() {
        mdist[cur].insert(query_color, cdist[query_color]);
        mnums[cur].insert(query_color, cnums[query_color]);
    }

    for &(nd, color, n_dist) in edges[cur].iter() {
        if nd != par {
            cdist[color] += n_dist;
            cnums[color] += 1;

            dfs2(edges, nd, cur, cdist, cnums, qs, mdist, mnums);

            cdist[color] -= n_dist;
            cnums[color] -= 1;
        }
    }
}

// LCA(Lowest Common Ancenstor)
fn lca(par: &Vec<Vec<usize>>, depth: &Vec<usize>, mut u:usize, mut v:usize) -> usize {
    if depth[u] > depth[v] {
        std::mem::swap(&mut u, &mut v);
    }

    let max_k = par.len();
    for k in 0..max_k {
        if ((depth[v] - depth[u]) >> k) & 1 > 0 {
            v = par[k][v];
        }
    }
    if u == v {
        return v
    }

    // 二分探索
    for k in (0..max_k).rev() {
        if par[k][u] != par[k][v] {
            u = par[k][u];
            v = par[k][v];
        }
    }
    par[0][u]
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      q: usize,
      abcds: [(usize1,usize1,usize1, usize); n-1],
      xyuvs: [(usize1,usize,usize1,usize1); q],
    }

    let logn = (n.next_power_of_two() as f64).log2() as usize;

    let mut edges = vec![vec![]; n];
    for (a, b, c, d) in abcds {
        edges[a].push((b, c, d));
        edges[b].push((a, c, d));
    }

    let mut par = vec![vec![0; n]; logn+1];
    let mut depth = vec![0; n];
    let mut dist = vec![0; n];

    dfs1(&edges, 0, n, &mut par, &mut depth, &mut dist, 0);

    for k in 1..logn+1 {
        for i in 0..n {
            if par[k-1][i] < n {
                par[k][i] = par[k-1][par[k-1][i]];
            } else {
                par[k][i] = n;
            }
        }
    }

    let mut qs = vec![HashSet::new(); n];
    for &(x, y, u, v) in xyuvs.iter() {
        qs[u].insert(x);
        qs[v].insert(x);
        let w = lca(&par, &depth, u, v);
        qs[w].insert(x);
    }

    let mut cdist = vec![0; n];
    let mut cnums = vec![0; n];
    let mut mdist = vec![HashMap::new(); n];
    let mut mnums = vec![HashMap::new(); n];
    dfs2(&edges, 0, n, &mut cdist, &mut cnums, &qs, &mut mdist, &mut mnums);

    for i in 0..q {
        let (x, y, u, v) = xyuvs[i];
        let w = lca(&par, &depth, u, v);

        let du = dist[u] - mdist[u][&x] + mnums[u][&x] * y;
        let dv = dist[v] - mdist[v][&x] + mnums[v][&x] * y;
        let dw = dist[w] - mdist[w][&x] + mnums[w][&x] * y;
        puts!("{}", du + dv - 2 * dw);
    }
}
