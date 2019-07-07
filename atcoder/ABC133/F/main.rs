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

type Pair = (usize,usize,usize,usize);

fn dfs(edges: &Vec<Vec<Pair>>, cur: usize, par: usize, e2: &mut Vec<Pair>, depth: usize) {
    for &(nd, c, d, _) in edges[cur].iter() {
        if nd != par {
            e2[nd] = (cur, c, d, depth+1);
            dfs(edges, nd, cur, e2, depth+1);
        }
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      q: usize,
      abcds: [(usize1,usize1,usize1, usize); n-1],
      xyuvs: [(usize1,usize,usize1,usize1); q],
    }

    let mut edges = vec![vec![]; n];
    for (a, b, c, d) in abcds {
        edges[a].push((b, c, d, 0));
        edges[b].push((a, c, d, 0));
    }
    let mut e2 = vec![(0,0,0,0); n];
    dfs(&edges, 0, n, &mut e2, 0);

    for i in 0..q {
        let (x, y, u, v) = xyuvs[i];

        let mut n1 = u;
        let mut n2 = v;
        let mut cur_d = min(e2[n1].3, e2[n2].3);
        let mut c1 = 0;
        let mut c2 = 0;

        loop {
            // debug!(n1, n2, cur_d, c1, c2);
            while e2[n1].3 > cur_d {
                if e2[n1].1 == x {
                    c1 += y;
                } else {
                    c1 += e2[n1].2;
                }
                n1 = e2[n1].0;
            }

            while e2[n2].3 > cur_d {
                if e2[n2].1 == x {
                    c2 += y;
                } else {
                    c2 += e2[n2].2;
                }
                n2 = e2[n2].0;
            }
            // debug!(n1, n2, cur_d, c1, c2);

            if n1 == n2 {
                puts!("{}", c1+c2);
                break;
            } else {
                cur_d -= 1;
            }
        }
    }
}
