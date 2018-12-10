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
      k: usize,
      aa: [usize; n],
    }
    let mut ss = vec![0; n+1];
    for i in 0..n {
        ss[i+1] = aa[i] + ss[i];
    }

    let mut rs = vec![];
    for l in 0..n {
        for r in l..n {
            let v = ss[r+1] - ss[l];
            rs.push(v);
        }
    }
    rs.sort();
    rs.reverse();

    let max_v = rs[0];
    let mut d = 1;
    while (1 << d+1) < max_v { d += 1 };

    let mut ans = 0;
    loop {
        let mut tmp = vec![];
        let v = 1 << d;
        for &r in &rs {
            if r & v >= 1 {
                tmp.push(r);
            };
        }
        if tmp.len() >= k {
            rs = tmp;
            ans += v;
        }

        if d == 0 {
            break;
        }
        d -= 1;
    }
    println!("{}", ans);
}
