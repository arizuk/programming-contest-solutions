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


#[derive(Debug)]
pub struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}
impl Graph {
    pub fn dfs(&self, cur: usize, from: usize, ex: &mut Vec<Option<f64>>, i: usize) -> f64 {
        if ex[cur].is_some() {
            return ex[cur].unwrap();
        }

        let mut ex_list = vec![];
        for &to in self.edges[cur].iter() {
            ex_list.push(
                self.dfs(to, cur, ex, i) + 1.0
            )
        }

        if i == cur {
            ex_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if ex_list.len() > 1 {
                ex_list.pop();
            }
        }

        let e = ex_list.iter().sum::<f64>() / ex_list.len() as f64;
        ex[cur] = Some(e);
        e
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
      sts: [(usize1, usize1); m]
    }

    let mut e = vec![vec![]; n];
    for (s, t) in sts {
        e[s].push(t);
    }
    let g = Graph { edges: e, n: n };

    let mut ans: f64 = (1usize << 50) as f64;
    for i in 0..n {
        let mut ex = vec![None; n];
        ex[n-1] = Some(0.0);
        let ex0 = g.dfs(0, n, &mut ex, i);
        if ex0 < ans {
            ans = ex0;
        }
    }
    puts!("{}\n", ans);
}
