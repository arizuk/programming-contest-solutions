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

use std::collections::VecDeque;
use std::collections::HashSet;

fn build_tree(n: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut q = VecDeque::new();
    q.push_back((0, n));
    let mut vis = vec![false; n];

    let mut tree_edges = vec![vec![]; n];
    while q.len() > 0 {
        let (node, par) = q.pop_front().unwrap();
        if vis[node] {
            continue;
        }
        vis[node] = true;
        if par < n {
            tree_edges[par].push(node);
        }
        for &nd in edges[node].iter() {
            if !vis[nd] {
                q.push_back((nd, node));
            }
        }
    }
    return tree_edges
}

fn dfs(edges: &Vec<Vec<usize>>, cur: usize, par: usize, used: &mut HashSet<(usize,usize)>, g: &Vec<Vec<usize>>, ans: &mut Vec<(usize,usize)>) {
    for &nd in edges[cur].iter() {
        dfs(edges, nd, cur, used, g, ans);
    }

    let mut cnt = 0;
    for &nd in g[cur].iter() {
        if nd != par {
            let pair = if cur < nd { (cur, nd) } else { (nd, cur) };
            if !used.contains(&pair) {
                // debug!(cur, par, nd, used);
                cnt += 1;
                used.insert(pair);
                ans.push((cur, nd));
            }
        }
    }
    if cnt%2 == 1 && cur != 0 {
        let pair = if cur < par { (cur, par) } else { (par, cur) };
        used.insert(pair);
        ans.push((cur, par));
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      abs: [(usize1,usize1); m],
    }
    if m%2 == 1 {
        return puts!("{}", "-1");
    }
    let mut edges = vec![vec![]; n];
    for &(a,b) in abs.iter() {
        edges[a].push(b);
        edges[b].push(a);
    }

    let tree_edges = build_tree(n, &edges);
    let mut used = HashSet::new();
    let mut ans = vec![];
    dfs(&tree_edges, 0, n, &mut used, &edges, &mut ans);

    for a in ans {
        // puts!("{} {}", a.0, a.1);
        puts!("{} {}", a.0+1, a.1+1);
    }
}
