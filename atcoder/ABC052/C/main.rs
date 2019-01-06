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

fn prime_factorization(mut n: usize) -> Vec<(usize, usize)> {
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

const MOD:usize = 1e9 as usize + 7;

fn main() {
    input!{
      n: usize,
    }
    let mut cnt = vec![0; 1001];

    for v in 2..n+1 {
        let divs = prime_factorization(v);
        for &(v, c) in divs.iter() {
            cnt[v] += c;
        }
    }

    let mut ans = 1;
    for i in 2..n+1 {
        ans *= cnt[i]+1;
        ans %= MOD;
    }
    println!("{}", ans);
}
