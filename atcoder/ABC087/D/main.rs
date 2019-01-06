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
      m: usize,
      lrds: [(usize, usize, i64); m],
    }
    let mut edges = vec![vec![]; n];

    for &(l, r, d) in lrds.iter() {
        edges[l-1].push((r-1, d));
        edges[r-1].push((l-1, -d));
    }

    const INF: i64 = 1 << 32;
    let mut xs = vec![INF; n];

    fn dfs(i: usize, edges: &Vec<Vec<(usize, i64)>>, xs: &mut Vec<i64>) -> bool {
        for &(ni, d) in edges[i].iter() {
            let nx = xs[i] + d;
            if xs[ni] == INF {
                xs[ni] = nx;

                if !dfs(ni, edges, xs) {
                    return false;
                }
            }

            if nx != xs[ni] {
                return false;
            } else {
                continue;
            }
        }
        true
    }

    for i in 0..n {
        if xs[i] == INF {
            xs[i] = 0;
            if !dfs(i, &edges, &mut xs) {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}
