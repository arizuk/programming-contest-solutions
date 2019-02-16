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

fn max_str<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() == b.len() {
        return if a < b { b } else { a }
    }
    if a.len() > b.len() { a } else { b }
}

fn main() {
    input!{
      n: usize,
      m: usize,
      aa: [usize; m],
    }
    // 1,2,3,4,5,6,7,8,9
    // 2,5,5,4,5,6,3,7,6
    let mut useable = [false; 10];
    for a in aa {
        useable[a as usize] = true;
    }
    if useable[7] {
        useable[6] = false;
        useable[9] = false;
    }
    if useable[9] {
        useable[6] = false;
    }

    if useable[1] {
        useable[4] = false;
    }

    if useable[5] {
        useable[3] = false;
        useable[2] = false;
    }

    if useable[3] {
        useable[2] = false;
    }

    let mut dp = vec!["".to_string(); n+1];
    let ok = |i, n, dp: &[String]| -> bool {
        i >= n && (i-n == 0 || dp[i-n] != "")
    };

    for i in 2..n+1 {
        if ok(i, 2, &dp) {
            if useable[1] {
                let string = dp[i-2].to_string() + "1";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
        }

        if ok(i, 3, &dp) {
            if useable[7] {
                let string = dp[i-3].to_string() + "7";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
        }

        if ok(i, 4, &dp) {
            if useable[4] {
                let string = dp[i-4].to_string() + "4";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
        }

        if ok(i, 5, &dp) {
            if useable[2] {
                let string = dp[i-5].to_string() + "2";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
            if useable[3] {
                let string = dp[i-5].to_string() + "3";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
            if useable[5] {
                let string = dp[i-5].to_string() + "5";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
        }

        if ok(i, 6, &dp) {
            if useable[9] {
                let string = dp[i-6].to_string() + "9";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
            if useable[6] {
                let string = dp[i-6].to_string() + "6";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
        }

        if ok(i, 7, &dp) {
            if useable[8] {
                let string = dp[i-7].to_string() + "8";
                dp[i] = (*max_str(&dp[i], &string)).clone()
            }
        }
    }
    println!("{}", dp[n]);
}
