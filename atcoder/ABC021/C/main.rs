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

fn main() {
    input!{
      n: usize,
      a: usize1,
      b: usize1,
      m: usize,
      xys: [(usize1,usize1); m]
    }

    let mut edges = vec![vec![]; n];
    for (x, y) in xys {
        edges[x].push(y);
        edges[y].push(x);
    }

    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut dist = vec![n+1; n];
    let mut dp = vec![0; n];
    let mut vis = vec![false; n];

    q.push_back(a);
    dist[a] = 0;
    dp[a] = 1;

    const MOD: usize = 1e9 as usize + 7;

    while q.len() > 0 {
        let node = q.pop_front().unwrap();
        if vis[node] {
            continue;
        }
        vis[node] = true;
        for &nx_node in edges[node].iter() {
            if dist[nx_node] >= dist[node] + 1 {
                dp[nx_node] += dp[node];
                dp[nx_node] %= MOD;
                dist[nx_node] = dist[node] + 1;
                q.push_back(nx_node);
            }
        }
    }
    println!("{}", dp[b]);
}
