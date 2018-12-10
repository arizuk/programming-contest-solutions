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

#[allow(unused_imports)]
use std::cmp::{min, max};

fn main() {
    input!{
        n: usize,
        ws: [(i64,i64,i64); n],
    }
    let mut xs = vec![0; n];
    let mut ys = vec![0; n];
    let mut hs = vec![0; n];
    for i in 0..n {
        xs[i] = ws[i].0;
        ys[i] = ws[i].1;
        hs[i] = ws[i].2;
    }

    for cx in 0i64..101 {
        for cy in 0i64..101 {
            let mut hh = 0;
            for i in 0..ws.len() {
                if hs[i] != 0 {
                    hh = hs[i] + (xs[i] - cx).abs() + (ys[i] - cy).abs();
                    break;
                }
            }

            let mut ok = true;
            for i in 0..ws.len() {
                if hs[i] != max(0, hh - (xs[i] - cx).abs() - (ys[i] - cy).abs()) {
                    ok = false;
                    break;
                }
            }

            if ok {
                println!("{} {} {}", cx, cy, hh);
            }
        }
    }
}
