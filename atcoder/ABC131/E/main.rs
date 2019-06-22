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

pub fn combination(n: u64, mut k: u64) -> u64 {
    assert!(n >= k);
    if k * 2 > n {
        k = n - k
    }
    let mut ans = 1;
    for d in 1..k + 1 {
        ans *= n + 1 - d;
        ans /= d;
    }
    ans
}

fn main() {
    input!{
      n: u64,
      k: u64,
    }
    if n == 2 {
        if k == 0 {
            println!("{}", 1);
            println!("{} {}", 1, 2);
        } else {
            println!("{}", -1);
        }
        return;
    }




    let cur = combination(n-1, 2);
    if k > cur {
        // debug!(cur);
        return println!("{}", -1);
    }

    let mut ans = vec![];
    for i in 2..n+1 {
        ans.push((1, i));
    }


    let mut a = 2;
    let mut b = 3;
    for i in 0..cur-k {
        ans.push((a, b));
        b += 1;
        if b > n {
            a += 1;
            b = a + 1;
        }
    }
    println!("{}", ans.len());
    for a in ans {
        println!("{} {}", a.0, a.1);
    }
}
