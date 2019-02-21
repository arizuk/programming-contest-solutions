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
      aa: [u64; n],
    }
    let mut even = vec![0; n+2];
    let mut odd = vec![0; n+2];
    let mut last_even = 0;
    let mut last_odd = 0;
    for i in 0..n {
        if i%2==0 {
            even[i+2] = even[i] + aa[i];
            last_even = i+2;
        } else {
            odd[i+2] = odd[i] + aa[i];
            last_odd = i+2;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        // iのポイントでわける
        // left
        let mut left_even = 0;
        let mut left_odd = 0;
        if i%2==0 {
            if i >= 1 {
                left_odd = odd[i-1+2];
            }
            if i >= 2 {
                left_even = even[i-2+2];
            }
        } else {
            if i >= 1 {
                left_even = even[i-1+2];
            }
            if i >= 2 {
                left_odd = odd[i-2+2];
            }
        }

        let mut right_even = 0;
        let mut right_odd = 0;
        if i%2==0 {
            if i<n-1 {
                right_even = odd[last_odd] - odd[i+1]
            }
            if i<n-2 {
                right_odd = even[last_even] - even[i+2];
            }
        } else {
            if i<n-1 {
                right_odd = even[last_even] - even[i+1]
            }
            if i<n-2 {
                right_even = odd[last_odd] - odd[i+2];
            }
        }

        let even = left_even+right_even;
        let odd = left_odd+right_odd;
        if even == odd {
            ans += 1;
        }
    }
    println!("{}", ans);
}

