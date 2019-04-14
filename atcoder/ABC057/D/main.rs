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
use std::collections::HashMap;
type I = usize;

pub fn combination(n: usize, mut k: usize) -> usize {
    assert!(n >= k);
    if k * 2 > n {
        k = n - k
    }
    let mut ans = 1;
    for d in 1..k + 1 {
        ans *= n + 1 - d;
        ans /= d;
    }
    ans
}

fn main() {
    input!{
      n: usize,
      a: usize,
      b: usize,
      mut vs: [usize; n],
    }
    let mut all = HashMap::new();
    for &v in vs.iter() {
        let e = all.entry(v).or_insert(0);
        *e += 1;
    }

    vs.sort();
    vs.reverse();
    let mut sum = 0;

    let mut used = HashMap::new();
    for i in 0..a {
        sum += vs[i];
        let e = used.entry(vs[i]).or_insert(0);
        *e += 1;
    }
    println!("{}", (sum as f64)/(a as f64));


    if used.keys().len() == 1 {
        let k = used.keys().nth(0).unwrap();
        let mut ans = 0;
        for i in a..b+1 {
            if i > all[k] {
                break;
            }
            let num = combination(all[k], i);
            ans += num;
        }
        println!("{}", ans);
    } else {
        let mut ans = 1;
        for (k, &v) in used.iter() {
            ans *= combination(all[k], v);
        }
        println!("{}", ans);
    }
}
