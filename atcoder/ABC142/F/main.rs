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

pub fn decompose(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut vs = Vec::new();
    let num_v = graph.len();
    let mut cmp = vec![0; num_v];

    let mut reverse_graph = vec![vec![]; num_v];
    for i in 0..num_v {
        for &v in graph[i].iter() {
            reverse_graph[v].push(i);
        }
    }
    let mut used = vec![false; num_v];

    let mut stack = VecDeque::new();
    let mut added = vec![false; num_v];
    for i in 0..num_v {
        if used[i] {
            continue;
        }
        stack.push_front(i);
        while let Some(v) = stack.pop_front() {
            stack.push_front(v);
            used[v] = true;
            let mut pushed = false;
            for &u in graph[v].iter().rev() {
                if !used[u] {
                    stack.push_front(u);
                    pushed = true;
                }
            }
            if !pushed {
                stack.pop_front();
                if !added[v] {
                    vs.push(v);
                    added[v] = true;
                }
            }
        }
    }

    used = vec![false; num_v];
    let mut k = 0;
    vs.reverse();
    for &i in vs.iter() {
        if used[i] {
            continue;
        }
        stack.push_front(i);
        used[i] = true;
        cmp[i] = k;
        while let Some(v) = stack.pop_front() {
            stack.push_front(v);
            let mut pushed = false;
            for &u in reverse_graph[v].iter() {
                if used[u] {
                    continue;
                }
                used[u] = true;
                cmp[u] = k;
                stack.push_front(u);
                pushed = true;
            }
            if !pushed {
                stack.pop_front();
            }
        }
        k += 1;
    }

    return cmp;
}

use std::collections::HashSet;

#[derive(Debug)]
pub struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}
impl Graph {
    pub fn bfs(&self, s: usize) -> Vec<i64> {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        let mut dist = vec![-1; self.n];
        dist[s] = 0;
        q.push_back(s);
        while let Some(cur) = q.pop_front() {
            for &adj in self.edges[cur].iter() {
                if dist[adj] == -1 {
                    dist[adj] = dist[cur] + 1;
                    q.push_back(adj);
                }
            }
        }
        dist
    }
}

fn solve(nodes: Vec<usize>, n:usize, e: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut set: HashSet<usize> = nodes.iter().map(|&v|v).collect();
    let mut g = vec![vec![]; n];
    for &(a, b) in e.iter() {
        if set.contains(&a) && set.contains(&b) {
            g[a].push(b);
        }
    }

    if nodes.iter().all(|&nd| g[nd].len() == 1) {
        return nodes;
    }

    let s = *nodes.iter().find(|&&nd| g[nd].len() != 1).unwrap();
    let mut q = VecDeque::new();
    let mut from = vec![-1; n];
    from[s] = 0;
    q.push_back(s);

    // Find the shortest loop
    while let Some(cur) = q.pop_front() {
        for &adj in g[cur].iter() {
            if adj == s {
                from[s] = cur as i64;
                break;
            }
            if from[adj] == -1 {
                from[adj] = cur as i64;
                q.push_back(adj);
            }
        }
    }

    let mut cur = from[s];
    let mut path = vec![];
    while cur as usize != s {
        path.push(cur as usize);
        cur = from[cur as usize];
    }
    path.push(cur as usize);
    return solve(path, n, e);
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
      abs: [(usize1, usize1); m],
    }
    let mut e = vec![vec![]; n];
    for &(a, b) in abs.iter() {
        e[a].push(b);
    }

    let components = decompose(&e);
    let mc = *components.iter().max().unwrap();
    let mut cs = vec![vec![]; mc+1];
    for i in 0..n {
        let c = components[i];
        cs[c].push(i);
    }

    for i in 0..cs.len() {
        let nodes = &cs[i];
        if nodes.len() == 1 {
            continue;
        }
        let ans = solve(nodes.clone(), n, &abs);
        puts!("{}\n", ans.len());
        for a in ans {
            puts!("{}\n", a+1);
        }
        return;
    }
    puts!("{}\n", -1);
}
