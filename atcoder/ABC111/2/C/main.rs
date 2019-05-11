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
      vs: [usize; n],
    }

    use std::collections::HashMap;
    let mut a = HashMap::new();
    let mut b = HashMap::new();

    for i in 0..n/2 {
        let x = i*2;
        let y = i*2+1;
        *a.entry(vs[x]).or_insert(0) += 1;
        *b.entry(vs[y]).or_insert(0) += 1;
    }
    let mut a: Vec<(usize, usize)> = a.iter().map(|(&a, &b)| (a, b)).collect();
    a.sort_by_key(|&(_, b)| b);
    a.reverse();
    let mut b: Vec<(usize, usize)> = b.iter().map(|(&a, &b)| (a, b)).collect();
    b.sort_by_key(|&(_, b)| b);
    b.reverse();

    if a.len() == 1 {
        a.push((0, 0));
    }
    if b.len() == 1 {
        b.push((0, 0));
    }

    if a[0].0 != b[0].0 {
        let (_, c1) = a[0];
        let (_, c2) = b[0];
        println!("{}", n-c1-c2);
    } else {
        let (_, c1) = a[0];
        let (_, c2) = b[1];
        let mut ans = n-c1-c2;

        let (_, c1) = a[1];
        let (_, c2) = b[0];
        ans = min(ans, n-c1-c2);
        println!("{}", ans);
    }
}
