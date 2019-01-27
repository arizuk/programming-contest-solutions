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
      xs: [usize; n],
      abys: [(usize, usize, usize); m]
    }
    let mut h = vec![0; n];
    for i in 0..m {
        let &(a, b, _y) = &abys[i];
        h[a-1] += 1;
        h[b-1] += 1;
    }
    let mut edges: Vec<(usize, usize, usize, usize)> = abys.iter().enumerate().map(|(i, &v)| (v.2, v.0-1, v.1-1, i)).collect();
    edges.sort();
    edges.reverse();

    let mut w = 0;
    for i in 0..n {
        if h[i] > 0 {
            w += xs[i];
        }
    }

    let mut ans = 0;
    for i in 0..m {
        let edge = edges[i];
        if edge.0 > w {
            // debug!(w, edge);
            let a = edge.1;
            let b = edge.2;
            h[a] -= 1;
            h[b] -= 1;
            ans += 1;
            if h[a] == 0 {
                w -= xs[a];
            }
            if h[b] == 0 {
                w -= xs[b];
            }
        } else {
            return println!("{}", ans);
        }
    }
}
