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

const MOD: usize = 1e9 as usize + 7;

fn mod_pow(n: usize) -> usize {
    let mut ans = 1;
    for _i in 0..n {
        ans *= 2;
        ans %= MOD;
    }
    ans
}

fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }
    let mut aa = aa;
    aa.sort();
    if n%2 == 0 {
        let mut e = 1;
        for i in 0..n/2 {
            if !(aa[2*i] == e && aa[2*i+1] == e) {
                return println!("{}", 0);
            }
            // debug!(n, 2*i, 2*i+1);
            e += 2;
        }
        println!("{}", mod_pow(n/2));
    } else {
        if aa[0] != 0 {
            return println!("{}", 0);
        }

        let mut e = 2;
        for i in 0..(n-1)/2 {
            if !(aa[2*i+1] == e && aa[2*i+2] == e) {
                return println!("{}", 0);
            }
            // debug!(n, 2*i+1, 2*i+2);
            e += 2;
        }
        println!("{}", mod_pow(n/2));
    }
}
