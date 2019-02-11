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

fn dfs(g: &[Vec<usize>], last: usize, d: usize, i: usize, wd: &mut [usize]) {
    wd[i] = d;
    for &n in &g[i] {
        if last != n {
            dfs(g, i, d+1, n, wd);
        }
    }
}

fn main() {
    input!{
      n: usize,
      abs: [(usize, usize); n-1],
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in abs.iter() {
        g[a-1].push(b-1);
        g[b-1].push(a-1);
    }

    let mut d1 = vec![0; n];
    let mut d2 = vec![0; n];
    dfs(&g, 0, 0, 0, &mut d1);
    dfs(&g, n-1, 0, n-1, &mut d2);

    let mut black = 0i64;
    for i in 0..n {
        if d1[i] <= d2[i] {
            black += 1;
        } else {
            black -= 1;
        }
    }
    if black > 0 {
        println!("{}", "Fennec")
    } else {
        println!("{}", "Snuke")
    }
}
