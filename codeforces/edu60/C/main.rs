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
      x1: i64,
      y1: i64,
      x2: i64,
      y2: i64,
      n: usize,
      s: chars,
    }
    let mut dx = 0;
    let mut dy = 0;
    for i in 0..n {
        match s[i] {
            'U' => dy+=1,
            'D' => dy-=1,
            'R' => dx+=1,
            'L' => dx-=1,
            _ => panic!(),
        }
    }

    let n = n as i64;
    let tx = x2-x1;
    let ty = y2-y1;


    let binsearch = |mut l: i64, mut r: i64| -> i64 {
        l -= 1;
        while r - l > 1 {
            let m = l + (r - l) / 2;
            let ddx = (tx - dx*m).abs();
            let ddy = (ty - dy*m).abs();
            if ddx + ddy <= n*m {
                r = m;
            } else {
                l = m;
            }
        }
        r
    };
    let limit = tx.abs() + ty.abs() + 1;
    let m = binsearch(0, limit);
    if m == limit {
        return println!("{}", -1);
    }

    let mut ddx = dx*m;
    let mut ddy = dy*m;
    let mut cnt = n*m;
    for i in (0..n as usize).rev() {
        if (tx - ddx).abs()+(ty - ddy).abs() > cnt {
            return println!("{}", cnt+1);
        }

        cnt -= 1;
        match s[i] {
            'U' => ddy-=1,
            'D' => ddy+=1,
            'R' => ddx-=1,
            'L' => ddx+=1,
            _ => panic!(),
        }
    }
    println!("{}", cnt+1);
}
