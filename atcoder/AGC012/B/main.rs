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

fn dfs(e: &Vec<Vec<usize>>, cur: usize, par: usize, d: usize, c: usize, dp: &mut Vec<Vec<bool>>, color: &mut Vec<usize>) {
    if dp[cur][d] {
        return;
    }
    for i in 0..d+1 {
        dp[cur][i] = true;
    }
    if color[cur] == 0 {
        color[cur] = c
    }
    if d == 0 {
        return;
    }
    for &nd in e[cur].iter() {
        if nd != par {
            dfs(e, nd, cur, d-1, c, dp, color);
        }
    }
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
      abs: [(usize1,usize1); m],
      q: usize,
      mut vdcs: [(usize1, usize, usize); q],
    }

    let mut e = vec![vec![]; n];
    for (a, b) in abs {
        e[a].push(b);
        e[b].push(a);
    }

    let mut dp = vec![vec![false; 11]; n];
    let mut color = vec![0; n];
    vdcs.reverse();
    for (v, d, c) in vdcs {
        dfs(&e, v, n, d, c, &mut dp, &mut color);
    }

    for i in 0..n {
        puts!("{}\n", color[i]);
    }
}
