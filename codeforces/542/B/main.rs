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


fn main() {
    input!{
      n: i64,
      aa: [i64; n*2],
    }

    use std::collections::HashMap;
    let mut map = HashMap::new();

    for i in 0..(2*n) {
        let a = aa[i as usize];
        let e  = map.entry(a).or_insert(vec![]);
        e.push(i);
    }

    let mut ans = 0;
    let mut p1 = 0;
    let mut p2 = 0;

    for x in 1..n+1 {
        let xs = map.get(&x).unwrap();

        let x1 = xs[0] as i64;
        let x2 = xs[1] as i64;

        debug!(p1, p2, x1, x2);

        if (x1-p1).abs() + (x2-p2).abs() <= (x1-p2).abs() + (x2-p1).abs() {
            ans += (x1-p1).abs() + (x2-p2).abs();
            p1 = x1;
            p2 = x2;
        } else {
            ans += (x1-p2).abs() + (x2-p1).abs();
            p1 = x2;
            p2 = x1;
        }
    }
    println!("{}", ans);
}
