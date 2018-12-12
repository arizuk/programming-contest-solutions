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

fn combi(n: u64, mut k: u64) -> u64 {
    assert!(n >= k);
    k = min(n-k, k);
    let mut ans = 1;
    for d in 1..k+1 {
        ans *= n+1-d;
        ans /= d;
    }
    ans
}

fn main() {
    input!{
      n: usize,
      p: u64,
      aa: [u64; n]
    }

    let mut even = 0u64;
    let mut odd = 0u64;

    for a in aa {
        if a%2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    let mut ans = 0;
    for e in 0..even+1 {
        for o in 0..odd+1 {
            if o%2 != p {
                continue;
            }
            ans += combi(even, e as u64) * combi(odd, o as u64)
        }
    }
    println!("{}", ans);
}
