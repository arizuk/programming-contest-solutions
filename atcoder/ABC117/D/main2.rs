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
      k: usize,
      aa: [usize; n],
    }
    let half = n/2 + n%2;
    let mut v_max = 0;
    for dd in (0..41).rev() {
        let v = 1 << dd;
        let mut cnt = 0;
        for i in 0..n {
            if aa[i] & v > 0 {
                cnt += 1;
            }
        }
        if cnt < half {
            v_max += v;
        }
    }

    let mut ans = 0;
    let k = k+1;
    for i in 0..41 {
        let mut v = 0;
        for j in i+1..41 {
            if k & (1 << j) > 0 {
                v += 1 << j;
            }
        }

        if k & (1 << i) == 0 {
            continue;
        }

        for j in 0..i {
            if v_max & (1 << j) > 0 {
                v += 1 << j;
            }
        }
        let mut xor = 0;
        for i in 0..n {
            xor += aa[i] ^ v;
        }
        ans = max(ans, xor);
    }
    println!("{}", ans);
}