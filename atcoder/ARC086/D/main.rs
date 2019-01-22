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

fn main() {
    input!{
      n: usize,
      aa: [i64; n],
    }
    let mut aa = aa;
    let max_v = *aa.iter().max().unwrap();
    let min_v = *aa.iter().min().unwrap();
    let v = if max_v.abs() > min_v.abs() { max_v } else { min_v };
    let vi = aa.iter().position(|&v2| v2 == v).unwrap();

    debug!(aa);
    let mut ops = vec![];
    for i in 0..n {
        if i != vi {
            aa[i] += v;
            ops.push((vi, i));
        }
        if aa[i] < 0 {
            aa[i] = -1 * aa[i];
        }
    }
    debug!(aa);

    let rev = v < 0;
    if rev {
        aa.reverse();
    }

    for i in 1..n {
        let a = aa[i-1];
        let b = aa[i];
        if a > b {
            aa[i] = aa[i] + a;

            if rev {
                ops.push((n-i, n-i-1));
            } else {
                ops.push((i-1, i));
            }
        }
    }
    debug!(aa);
    println!("{}", ops.len());
    for &(x, y) in ops.iter() {
        println!("{} {}", x+1, y+1);
    }
}
