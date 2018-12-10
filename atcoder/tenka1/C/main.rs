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
      aa: [i64; n],
    }
    let mut aa = aa;
    aa.sort();

    let mut s = vec![0; n];
    let mut j = aa.len() - 1;
    let mut k = 0;
    for i in 1..n {
        if i % 2 == 1 {
            s[i] = aa[j];
            j -= 1;
        } else {
            s[i] = aa[k];
            k += 1;
        }
    }
    s[0] = aa[k];
    let mut ans1 = 0;
    for i in 0..n-1 {
        ans1 += (s[i+1] - s[i]).abs();
    }


    let mut j = aa.len() - 1;
    let mut k = 0;
    for i in 1..n {
        if i % 2 == 1 {
            s[i] = aa[k];
            k += 1;
        } else {
            s[i] = aa[j];
            j -= 1;
        }
    }
    s[0] = aa[k];
    let mut ans2 = 0;
    for i in 0..n-1 {
        ans2 += (s[i+1] - s[i]).abs();
    }

    println!("{}", max(ans1, ans2));
}
