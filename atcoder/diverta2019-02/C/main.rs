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

fn main() {
    input!{
      n: usize,
      mut aa: [i64; n],
    }
    aa.sort();
    if aa.len() == 1 {
        return println!("{}", aa[0]);
    }
    if aa.len() == 2 {
        println!("{}", aa[1] - aa[0]);
        println!("{} {}", aa[1], aa[0]);
        return;
    }

    let mut ps = vec![];
    let mut ms = vec![];
    for a in aa {
        if a>=0 {
            ps.push(a);
        } else {
            ms.push(a);
        }
    }
    ps.sort();
    ms.sort();
    ms.reverse();
    let mut ans = vec![];
    if ms.len() == 0 {
        ms.push(ps[0]-ps[1]);
        ans.push((ps[0], ps[1]));
        ps.remove(0);
        ps.remove(0);
    } else if ps.len() == 0 {
        ps.push(ms[0]-ms[1]);
        ans.push((ms[0], ms[1]));
        ms.remove(0);
        ms.remove(0);
    }

    let m = ms[0];
    let p = ps[0];
    let mut cur = m;
    ms.remove(0);
    ps.remove(0);
    for p in ps {
        ans.push((cur, p));
        cur -= p;
    }
    ans.push((p, cur));
    cur = p - cur;
    for m in ms {
        ans.push((cur, m));
        cur -= m;
    }

    println!("{}", cur);
    for (a,b) in ans {
        println!("{} {}", a, b);
    }
}
