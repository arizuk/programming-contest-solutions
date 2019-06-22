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

fn dfs(edges: &Vec<Vec<usize>>, cur: usize, par:usize, root:usize, g: &mut Vec<usize>, ini: usize) {
    // treeじゃない場合は、DFSするときにすで探索済みかチェックが必要
    if g[cur] != ini {
        return;
    }
    g[cur] = root;
    for &node in edges[cur].iter() {
        if node != par {
            dfs(edges, node, cur, root, g, ini);
        }
    }
}


fn main() {
    input!{
      n: usize,
      xys: [(usize, usize); n],
    }
    use std::collections::HashMap;
    let mut xs: Vec<_> = xys.iter().map(|v|v.0).collect();
    xs.sort();
    xs.dedup();
    let xs: HashMap<_, _> = xs.into_iter().enumerate().map(|(i, v)| (v, i)).collect();

    let mut ys: Vec<_> = xys.iter().map(|v|v.1).collect();
    ys.sort();
    ys.dedup();
    let ys: HashMap<_, _> = ys.into_iter().enumerate().map(|(i, v)| (v, i)).collect();

    let xn = xs.len();
    let yn = ys.len();
    let m = xn+yn;
    let mut edges = vec![vec![]; m];
    for &(x, y) in xys.iter() {
         let a = xs[&x];
         let b = ys[&y] + xn;
         edges[a].push(b);
         edges[b].push(a);
    }

    let mut grp = vec![m; m];
    for i in 0..m {
        if grp[i] == m {
            dfs(&edges, i, m, i, &mut grp, m);
        }
    }
    let mut grp2: Vec<_> = grp.iter().cloned().collect();
    grp2.sort();
    grp2.dedup();
    let mut g_nodes = vec![vec![]; grp2.len()];
    for &(x, y) in xys.iter() {
         let a = xs[&x];
         let g = grp2.binary_search(&grp[a]).unwrap();
         g_nodes[g].push((x, y));
    }

    let mut ans = 0u64;
    for nodes in g_nodes {
        let mut xs: Vec<_> = nodes.iter().map(|v|v.0).collect();
        xs.sort();
        xs.dedup();
        let mut ys: Vec<_> = nodes.iter().map(|v|v.1).collect();
        ys.sort();
        ys.dedup();
        ans += xs.len() as u64 * ys.len() as u64;
    }
    println!("{}", ans-n as u64);
}
