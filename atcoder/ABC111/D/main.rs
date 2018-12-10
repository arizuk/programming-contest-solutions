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

#[allow(unused_imports)]
use std::cmp;

fn main() {
    input!{
        n: usize,
        xs: [[isize; 2]; n],
    }
    let r = (xs[0][0].abs() + xs[0][1].abs()) % 2;
    for i in 1..xs.len() {
        if r != (xs[i][0].abs() + xs[i][1].abs()) % 2 {
            println!("{}", -1);
            return;
        }
    }
    let mut max_d = 0;
    for i in 0..xs.len() {
        max_d = cmp::max(max_d, xs[i][0].abs() + xs[i][1].abs())
    }

    let m = max_d as usize;
    let mut strs = vec![];
    println!("{}", m);
    for _ in 0..m {
        strs.push("1");
    }
    println!("{}", strs.join(" "));

    for i in 0..xs.len() {
        let x = xs[i][0];
        let y = xs[i][1];
        let mut modes = String::new();
        if x > 0 {
            for _ in 0..x { modes += "R" }
        }
        if x < 0 {
            for _ in 0..x.abs() { modes += "L" }
        }
        if y > 0 {
            for _ in 0..y.abs() { modes += "U" }
        }
        if y < 0 {
            for _ in 0..y.abs() { modes += "D" }
        }
        for _ in 0..(m - modes.len()) / 2 {
            modes += "LR";
        }
        println!("{}", modes);
    }
}
