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

fn main() {
    input!{
      a: i64,
      b: i64,
      c: i64,
      d: i64,
      e: i64,
      f: i64,
    }
    let mut ans = 0f64;
    let mut ans_a = 0;
    let mut ans_s = 0;
    for i in 0..31 {
        for j in 0..31 {
            let aa = 100 * a * i;
            let bb = 100 * b * j;
            let sugar = f - aa - bb;
            if sugar < 0 { break; }
            let ee = (aa + bb) / 100 * e;
            let max_s = min(sugar, ee);
            for k in 0..(max_s/c+1) {
                let cc = k * c;
                let dd = (max_s - cc)/d*d;
                let ff = aa + bb + cc + dd;
                if ff == 0 { continue };
                let dense = (100 * (cc + dd)) as f64 / ff as f64;
                ans = if dense > ans { dense } else { ans };
                if ans == dense {
                    ans_a = ff;
                    ans_s = cc + dd;
                }
                // debug!(f, aa, bb, sugar, ee, max_s, ff, cc+dd);
                // println!("dense={:?}", dense);
            }
        }
    }
    println!("{} {}", ans_a, ans_s);
}
