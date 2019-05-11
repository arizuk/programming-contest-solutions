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
      xys: [(i64,i64); n],
    }
    let rem = (xys[0].0 + xys[0].1).abs() % 2;
    for &(x, y) in xys.iter() {
        if (x +y).abs()%2 != rem {
            return println!("{}", -1);
        }
    }
    let mut ds = vec![];
    for i in (0..32).rev() {
        ds.push(2i64.pow(i));
    }
    if rem == 0 {
        ds.push(1);
    }
    println!("{}", ds.len());
    for &d in ds.iter() {
        print!("{} ", d);
    }
    println!();

    for &(x, y) in xys.iter() {
        let u = x+y;
        let v = x-y;

        let mut usum = 0;
        let mut vsum = 0;

        let mut ans = vec![];
        for &d in ds.iter() {
            let udir;
            let vdir;
            if usum < u {
                usum += d;
                udir = 1;
            } else {
                usum -= d;
                udir = -1;
            }

            if vsum < v {
                vsum += d;
                vdir = 1;
            } else {
                vsum -= d;
                vdir = -1;
            }

            if udir == 1 && vdir == 1 { ans.push('R') }
            if udir == 1 && vdir == -1 { ans.push('U') }
            if udir == -1 && vdir == 1 { ans.push('D') }
            if udir == -1 && vdir == -1 { ans.push('L') }
        }
        assert_eq!(vsum, v);
        assert_eq!(usum, u);
        println!("{}", ans.into_iter().collect::<String>());
    }
}
