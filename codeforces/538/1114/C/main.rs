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
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;

fn prime_factorization(mut n: u64) -> Vec<(u64, u64)> {
    let mut i = 2;
    let mut ans = vec![];
    while i*i <= n {
        let mut cnt = 0;
        while n%i == 0 {
            n /= i;
            cnt += 1;
        }
        if cnt > 0 {
            ans.push((i, cnt));
        }
        i += 1;
    }
    if n > 1 {
        ans.push((n, 1));
    }
    ans
}

fn fact_num(n: u64, v: u64) -> u64 {
    if n < v {
        return 0;
    }
    let div = n/v;
    div + fact_num(div, v)
}


fn main() {
    input!{
      n: u64,
      b: u64,
    }
    let facts = prime_factorization(b);
    let mut ans: i64 = -1;
    for i in 0..facts.len() {
        let (v, pow) = facts[i];
        let fnum = fact_num(n, v);
        if ans == -1 {
            ans = (fnum/pow) as i64;
        } else {
            ans = min(ans, (fnum/pow) as i64);
        }
    }
    println!("{}", ans);
}
