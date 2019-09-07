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
use std::io::{stdout, stdin, BufWriter, Write};

fn encode(a: usize, b:usize, n:usize) -> usize {
    let idx = n*a + b - (a+1) * (a+2) / 2;
    idx
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      aa: [[usize1; n-1]; n],
    }
    let m = n*(n-1)/2;
    let mut edges = vec![vec![]; n*(n-1)/2];
    let mut h = vec![0; m];
    for i in 0..n {
        let a = i;
        for j in 0..n-2 {
            let mut a1 = a;
            let mut b1 = aa[i][j];
            if a1 > b1 {
                std::mem::swap(&mut a1, &mut b1);
            }

            let mut a2 = a;
            let mut b2 = aa[i][j+1];
            if a2 > b2 {
                std::mem::swap(&mut a2, &mut b2);
            }

            let i1 = encode(a1, b1, n);
            let i2 = encode(a2, b2, n);
            edges[i1].push(i2);
        }
    }
    for v in edges.iter_mut() {
        v.sort();
        v.dedup();
    }

    for i in 0..m {
        for &e in edges[i].iter() {
            h[e] += 1;
        }
    }
    use std::collections::VecDeque;
    let mut st = VecDeque::new();
    for i in 0..m {
        if h[i] == 0 {
            st.push_back((i, 0));
        }
    }

    let mut ans = 0;
    while let Some((i, d)) = st.pop_front() {
        ans = max(ans, d);
        for &e in edges[i].iter() {
            h[e] -= 1;
            if h[e] == 0 {
                st.push_back((e, d+1));
            }
        }
    }

    if h.iter().all(|&v| v == 0) {
        puts!("{}\n", ans+1);
    } else {
        puts!("{}\n", -1);
    }
}
