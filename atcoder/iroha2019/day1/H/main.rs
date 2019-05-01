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
type I = usize;

fn to_int(ns: &Vec<usize>) -> usize {
    let mut base = 1;
    let mut ret = 0;
    for i in (0..ns.len()).rev() {
        ret += ns[i] * base;
        base *= 10;
    }
    ret
}

fn main() {
    input!{
      ns: chars,
    }
    let mut ns: Vec<usize> = ns.iter().map(|v| (*v as u8 - '0' as u8) as usize).collect();
    let old = to_int(&ns);


    let mut left = 0;
    let mut right = ns.len()-1;

    loop {
        while ns[left] == 0 {
            left += 1;
            if left >= right {
                break;
            }
        }
        if left >= right { break; }
        while ns[right] == 9 {
            right -= 1;
            if left >= right {
                break;
            }
        }
        if left >= right { break; }
        let l = ns[left];
        let r = ns[right];
        let mv = min(9-r, l);
        ns[right] += mv;
        ns[left] -= mv;
    }

    let new = to_int(&ns);
    if old == new {
        if ns[0] == 9 || ns.len() == 1 {
            // 999 or 5
            ns[0] -= 1;
            ns.insert(0, 1);
        } else {
            // n99
            ns[0] += 1;
            ns[1] -= 1;
        }
    }
    let new = to_int(&ns);
    println!("{}", new);
}
