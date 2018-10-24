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
      n: usize,
      aa: [usize; n],
      bb: [usize; n],
      cc: [usize; n],
    }
    let mut aa = aa;
    let mut bb = bb;
    let mut cc = cc;
    aa.sort();
    bb.sort();
    cc.sort();

    // println!("aa={:?}", aa);
    // println!("bb={:?}", bb);
    // println!("cc={:?}", cc);

    let mut ans = 0;
    for i in 0..n {
        let b = bb[i];
        // 同じ値が複数あった場合の挙動が適当..
        let lower = match aa.binary_search(&b) {
            Ok(i) => i,
            Err(i) => i,
        };
        let higher = match cc.binary_search(&(b+1)) {
            Ok(i) => i,
            Err(i) => i,
        };
        ans += lower * (n - higher);
        // debug!(lower, higher, b, lower * (n - higher));
    }
    println!("{}", ans);
}
