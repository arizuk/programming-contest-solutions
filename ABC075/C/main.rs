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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn dfs(n: usize, edges: &[[bool; 50]], visited: &mut [bool], v: usize) {
    visited[v] = true;
    for i in 0..n {
        if edges[v][i] && visited[i] == false {
            dfs(n, edges, visited, i);
        }
    }
}

fn main() {
    input!{
      n: usize,
      m: usize,
      abs: [[usize; 2]; m],
    }

    let mut edges = [[false; 50]; 50];
    for i in 0..m {
        let a = abs[i][0] - 1;
        let b = abs[i][1] - 1;
        edges[a][b] = true;
        edges[b][a] = true;
    }

    let mut ans = 0;
    for i in 0..m {
        let a = abs[i][0] - 1;
        let b = abs[i][1] - 1;

        edges[a][b] = false;
        edges[b][a] = false;

        let mut visited = vec![false; n];
        dfs(n, &edges, &mut visited, 0);

        let mut bridge = false;
        for i in 0..n {
            if visited[i] == false { bridge = true; }
        }
        if bridge { ans += 1; }

        edges[a][b] = true;
        edges[b][a] = true;
    }
    println!("{}", ans);
}
