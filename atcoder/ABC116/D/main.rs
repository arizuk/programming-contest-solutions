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
use std::collections::VecDeque;

fn main() {
    input!{
      n: usize,
      k: usize,
      tds: [(usize, usize); n],
    }
    let mut tds = tds;
    tds.sort_by_key(|&(t, d)| d);
    tds.reverse();

    let mut ate = vec![false; n+1];
    let mut kind = 0;
    let mut ans = 0;
    let mut s1 = VecDeque::new();
    let mut s2 = VecDeque::new();
    for i in 0..k {
        let (t, d) = tds[i];
        if ate[t] == false {
            ate[t] = true;
            kind += 1;
        } else {
            s1.push_back((t, d));
        }
        ans += d;
    }
    for i in k..n {
        let (t, d) = tds[i];
        if ate[t] == false {
            ate[t] = true;
            s2.push_back((t, d));
        }
    }
    ans += kind*kind;

    let mut v = ans;
    while let Some((t, d)) = s1.pop_back() {
        v -= d;
        if let Some((t1, d1)) = s2.pop_front() {
            kind += 1;
            v += d1 + kind*kind - (kind-1)*(kind-1);
            ans = max(v, ans);
        } else {
            break;
        }
    }
    println!("{}", ans);
}
