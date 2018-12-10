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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn solve(ls: &Vec<usize>, ps: &Vec<usize>, x: usize, lv: usize) -> usize {
    if lv == 0 {
        return if x >= 1 { 1 } else { 0 };
    }
    if x == 0 {
        return 0;
    }

    if x <= ls[lv-1] + 1 {
        solve(ls, ps, x-1, lv-1)
    } else {
        1 + solve(ls, ps, x-ls[lv-1]-2, lv-1) + ps[lv-1]
    }
}

fn main() {
    input!{
      n: usize,
      x: usize,
    }

    let mut ls = vec![0usize; n+1];
    let mut ps = vec![0usize; n+1];
    ls[0] = 1;
    ps[0] = 1;

    for i in 1..n+1 {
        ls[i] = ls[i-1]*2 + 3;
        ps[i] = ps[i-1]*2 + 1;
    }
    println!("{}", solve(&ls, &ps, x, n));
}
