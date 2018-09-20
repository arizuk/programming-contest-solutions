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

use std::cmp;
fn cost(xs: &[i32]) -> usize {
    let f = xs[0];
    let l = xs[xs.len() - 1];

    if f < 0 && l > 0 {
        let len = l - f + cmp::min(l.abs(), f.abs());
        // println!("{:?} {}", xs, len);
        return len as usize;
    } else if f > 0 {
        return l as usize;
    } else {
        return (f * -1) as usize;
        // println!("{:?} {}", xs, f * -1);
    }
}

fn main() {
    input!{
        n: i32,
        k: i32,
        xs: [i32; n]
    }
    // println!("{:?}", xs);

    let mut min = 0;
    for i in 0..(xs.len() - k as usize + 1) {
        let e = i + k as usize;
        if min == 0 {
            min = cost(&xs[i..e]);
        } else {
            min = cmp::min(cost(&xs[i..e]), min);
        }
    }
    println!("{:?}", min);
}
