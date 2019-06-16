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

fn solve(n: i64, a: &[i64], b: &[i64]) -> i64 {
    let mut uabs: Vec<_> = a.iter().zip(b.iter()).map(|(&a, &b)|
        (
            if b > a { n/a } else { 0 },
            a,
            b
        )
    ).collect();
    uabs.sort();

    let mut ans = n;
    for i in 0..uabs[0].0+1 {
        for j in 0..uabs[1].0+1 {
            let &(_, ai, _) = &uabs[0];
            let &(_, aj, _) = &uabs[1];
            let &(_, ak, bk) = &uabs[2];

            let rem = n - ai*i - aj*j;
            if rem < 0 {
                break;
            }
            let k =  if bk > ak { rem/ak } else { 0 };

            let incr = [i,j,k].into_iter().enumerate().map(|(i, n)| {
                let b = uabs[i].2;
                let a = uabs[i].1;
                n * (b - a)
            }).sum::<i64>();

            ans = max(
                ans,
                n + incr,
            );
        }
    }
    ans
}

fn main() {
    input!{
      n: i64,
      a: [i64; 3],
      b: [i64; 3],
    }
    let m = solve(n, &a, &b);
    let ans = solve(m, &b, &a);
    println!("{}", ans);
}
