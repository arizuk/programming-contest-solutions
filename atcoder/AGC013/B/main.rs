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

fn dfs1(e: &Vec<Vec<usize>>,cur:usize, par:usize, seen: &mut Vec<bool>) -> Option<usize> {
    seen[cur] = true;

    let mut stopped = true;
    for &n in e[cur].iter() {
        if !seen[n] && n != par {
            stopped = false;
            let ret = dfs1(e, n, cur, seen);
            if ret.is_some() {
                return ret;
            }
        }
    }

    if stopped {
        return Some(cur)
    } else {
        return None
    }
}

fn dfs2(e: &Vec<Vec<usize>>,cur:usize, par:usize, seen: &mut Vec<bool>, path: &mut Vec<usize>, vs: &Vec<usize>) -> bool {
    seen[cur] = true;
    path.push(cur);
    let mut stopped = true;
    for &n in e[cur].iter() {
        if !seen[n] && n != par {
            stopped = false;
            if dfs2(e, n, cur, seen, path, vs) {
                return true;
            }
        }
    }

    if stopped {
        if vs.iter().all(|&v|seen[v]) {
            return true;
        }
    }
    path.pop();
    return false;
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
    for (a, b) in abs {
        e[a].push(b);
        e[b].push(a);
    }

    let mut seen = vec![false ; n];
    let n1 = dfs1(&e, 0, n, &mut seen).unwrap();

    let mut seen = vec![false ; n];
    let vs = e[n1].clone();
    let mut path = vec![];
    dfs2(&e, n1, n, &mut seen, &mut path, &vs);

    puts!("{}\n", path.len());
    for i in 0..path.len() {
        if i == path.len() - 1 {
            puts!("{}", path[i]+1);
        } else {
            puts!("{} ", path[i]+1);
        }
    }
    puts!("\n")
}
