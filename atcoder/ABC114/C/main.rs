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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn to753(mut n: usize) -> usize {
    let mut ans = 0;
    let mut d = 1;
    let mut ok3 = false;
    let mut ok5 = false;
    let mut ok7 = false;
    loop {
        ans = match n%4 {
            3 => {
                ok7 = true;
                ans + 7 * d
            },
            2 => {
                ok5 = true;
                ans + 5 * d
            },
            1 => {
                ok3 = true;
                ans + 3 * d
            },
            0 => return 0,
            _ => panic!("error")
        };
        if n <= 3 {
            break;
        }
        n /= 4;
        d *= 10;
    }
    if ok3 && ok5 && ok7 {
        ans
    } else {
        0
    }
}

fn main() {
    input!{
      n: usize,
    }
    let mut ans = 0;
    let mut i = 0;
    loop {
        let n753 = to753(i);
        if n753 > n {
            break;
        }
        if to753(i) != 0 {
            ans += 1;
        }
        i += 1;
    }
    println!("{}", ans);
}
