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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      a: usize,
      b: usize,
      g0: u64,
      x: u64,
      y: u64,
      z: u64,
    }

    let mut hs = vec![];
    let mut g = g0;
    for _ in 0..n*m {
        hs.push(g);
        g = (g * x  + y) % z;
    }

    use std::collections::BTreeMap;


    const INF: u64 = 1 << 60;
    let mut mins = vec![vec![INF; m]; n];

    for i in 0..n {
        let mut bs = BTreeMap::new();
        for j in 0..b {
            let idx = i*m + j;
            *bs.entry(hs[idx]).or_insert(0) += 1;
        }

        for j in 0..m-b+1 {
            if j > 0 {
                // Update map
                let idx = i*m + j-1;
                let cnt = bs[&hs[idx]];
                if cnt == 1 {
                    bs.remove(&hs[idx]);
                } else {
                    *bs.entry(hs[idx]).or_insert(0) -= 1;
                }

                let idx = i*m + j+b-1;
                *bs.entry(hs[idx]).or_insert(0) += 1;
            }
            let (&v, _) = bs.iter().next().unwrap();
            mins[i][j] = v;
        }
    }

    // debug!(n, m, a, b);
    // for i in 0..n {
    //     for j in 0..m {
    //         print!("{} ", hs[ n*i + j ])
    //     }
    //     println!();
    // }
    // println!("{}", "-------------------------");

    // for i in 0..n {
    //     for j in 0..m {
    //         if mins[i][j] == INF {
    //             print!("{} ", -1);
    //         } else {
    //             print!("{} ", mins[i][j]);
    //         }
    //     }
    //     println!();
    // }

    let mut ans = 0;
    for j in 0..m-b+1 {
        let mut bs = BTreeMap::new();
        for i in 0..a {
            *bs.entry(mins[i][j]).or_insert(0) += 1;
        }

        for i in 0..n-a+1 {
            if i > 0 {
                // Update map
                let prev = mins[i-1][j];
                let cnt = bs[&prev];
                if cnt == 1 {
                    bs.remove(&prev);
                } else {
                    *bs.entry(prev).or_insert(0) -= 1;
                }

                *bs.entry(mins[i + a - 1][j]).or_insert(0) += 1;
            }

            let (&v, _) = bs.iter().next().unwrap();

            ans += v;
        }
    }
    puts!("{}", ans);
}
