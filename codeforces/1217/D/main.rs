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

fn dfs(e: &Vec<Vec<usize>>, cur: usize, vs: &mut Vec<usize>, seen: &mut Vec<bool>) {
    if seen[cur] {
        return;
    }
    seen[cur] = true;
    for &next in e[cur].iter() {
        dfs(e, next, vs, seen);
    }
    vs.push(cur);
}

fn rdfs(e: &Vec<Vec<usize>>, cur: usize, k: usize, cmp: &mut Vec<usize>, seen: &mut Vec<bool>) {
    if seen[cur] {
        return;
    }
    debug!(cur, k);
    seen[cur] = true;
    cmp[cur] = k;
    for &next in e[cur].iter() {
        rdfs(e, next, k, cmp, seen);
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
      uvs: [(usize1,usize1); m],
    }

    let mut e = vec![vec![]; n];
    let mut re = vec![vec![]; n];
    for (u, v) in uvs {
        e[u].push(v);
        re[v].push(u);
    }

    let mut vs = vec![];
    let mut seen = vec![false; n];
    for i in 0..n {
        dfs(&e, i, &mut vs, &mut seen);
    }

    vs.reverse();
    let mut k = 0;
    let mut cmp = vec![0; n];
    let mut seen = vec![false; n];
    for &v in vs.iter() {
        rdfs(&re, v, k, &mut cmp, &mut seen);
        k += 1;
    }

    let mut cmp2 = cmp.clone();
    cmp2.sort();
    cmp2.dedup();
    if cmp2.len() == cmp.len() {
        puts!("{}\n", 1);
        for _ in 0..n {
            puts!("{} ", 1);
        }
        puts!("\n");
        return;
    }
}
