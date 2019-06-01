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

fn dfs(edges: &Vec<Vec<usize>>, cur: usize, par: usize, ans: &mut Vec<usize>, cs: &Vec<usize>, idx: &mut usize) {
    ans[cur] = cs[*idx];
    for &next in edges[cur].iter() {
        if next != par {
            *idx += 1;
            dfs(edges, next, cur, ans, cs, idx);
        }
    }
}

fn main() {
    input!{
      n: usize,
      abs: [(usize,usize); n-1],
      mut cs: [usize; n]
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in abs {
        edges[a-1].push(b-1);
        edges[b-1].push(a-1);
    }
    cs.sort();
    let mut sum = &cs[0..n-1].iter().sum::<usize>();
    debug!(cs);
    println!("{}", sum);
    cs.reverse();
    let mut ans = vec![0; n];
    dfs(&edges, 0, n, &mut ans, &cs, &mut 0);
    for a in ans {
        print!("{} ", a);
    }
    println!();
}
