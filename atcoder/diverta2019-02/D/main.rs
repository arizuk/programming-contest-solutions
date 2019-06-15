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
      n: u64,
      gsbs: [(u64,u64,u64); 2],
    }

    let (ga1, sa1, ba1) = gsbs[0];
    let (ga2, sa2, ba2) = gsbs[1];

    let mut pairs = vec![
        (ga2 * sa1 * ba1, ga1, ga2),
        (ga1 * sa2 * ba1, sa1, sa2),
        (ga1 * sa1 * ba2, ba1, ba2),
    ];
    pairs.sort();
    pairs.reverse();
    // debug!(pairs);

    let mut n = n;
    let mut temp = 0;
    for &(_, a1, a2) in pairs.iter() {
        if a2 > a1 {
            temp += (n/a1) * a2;
            n = n%a1;
            // debug!(temp, n, a1, a2);
        }
    }
    n += temp;

    let mut pairs = vec![
        (ga1 * sa2 * ba2, ga2, ga1),
        (ga2 * sa1 * ba2, sa2, sa1),
        (ga2 * sa2 * ba1, ba2, ba1),
    ];
    pairs.sort();
    pairs.reverse();
    // debug!(pairs);
    // debug!(n);

    let mut temp = 0;
    for &(_, a1, a2) in pairs.iter() {
        if a2 > a1 {
            temp += (n/a1) * a2;
            n = n%a1;
            // debug!(temp, n, a1, a2);
        }
    }
    n += temp;
    println!("{}", n);
}
