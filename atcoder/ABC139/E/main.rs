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

fn dfs(edges: &Vec<Vec<usize>>, cur: usize, memo: &mut Vec<i64>, seen: &mut Vec<bool>) -> bool {
    if memo[cur] != -1 {
        return true;
    }
    if seen[cur] {
        return false
    }
    seen[cur] = true;
    let mut ret = 0;
    for &next in edges[cur].iter() {
        if !dfs(edges, next, memo, seen) {
            return false;
        }
        ret = max(ret, memo[next]);
    }
    memo[cur] = ret + 1;
    return true;
}

fn encode(a: usize, b:usize, n:usize) -> usize {
    let idx = n*a + b - (a+1) * (a+2) / 2;
    idx
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      aa: [[usize1; n-1]; n],
    }
    let mut edges = vec![vec![]; n*(n-1)/2];

    for i in 0..n {
        let a = i;
        for j in 0..n-2 {
            let mut a1 = a;
            let mut b1 = aa[i][j];
            if a1 > b1 {
                std::mem::swap(&mut a1, &mut b1);
            }

            let mut a2 = a;
            let mut b2 = aa[i][j+1];
            if a2 > b2 {
                std::mem::swap(&mut a2, &mut b2);
            }

            let i1 = encode(a1, b1, n);
            let i2 = encode(a2, b2, n);
            edges[i1].push(i2);
        }
    }

    for v in edges.iter_mut() {
        v.sort();
        v.dedup();
    }

    let mut memo = vec![-1; n*(n-1)/2];
    let mut seen = vec![false; n*(n-1)/2];
    for mut a in 0..n {
        let mut b = aa[a][0];
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        let s = encode(a, b, n);
        if !dfs(&edges, s, &mut memo, &mut seen) {
            return puts!("{}\n", -1);
        }
    }
    let ans = *memo.iter().max().unwrap();
    puts!("{}\n", ans);
}
