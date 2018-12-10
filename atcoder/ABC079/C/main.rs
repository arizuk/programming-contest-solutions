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

fn op(v: usize) -> char {
    if v == 0 { '+' } else { '-' }
}

fn main() {
    input!{
        abcd: chars
    }
    let a = abcd[0] as i64 - '0' as i64;
    let b = abcd[1] as i64 - '0' as i64;
    let c = abcd[2] as i64 - '0' as i64;
    let d = abcd[3] as i64 - '0' as i64;

    for ops1 in 0..2 {
        for ops2 in 0..2 {
            for ops3 in 0..2 {
                let mut e = a;
                if ops1 == 0 { e += b } else { e -= b }
                if ops2 == 0 { e += c } else { e -= c }
                if ops3 == 0 { e += d } else { e -= d }
                if e == 7 {
                    println!("{}{}{}{}{}{}{}=7", a, op(ops1), b, op(ops2), c, op(ops3), d);
                    return;
                }
            }
        }
    }
}
