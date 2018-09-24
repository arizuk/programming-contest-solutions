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
        t: [[usize; 3]; n],
    }
    let mut t = t.to_vec();
    t.insert(0, vec![0, 0, 0]);

    for i in 1..t.len() {
        let dt = t[i][0] as i32 - t[i-1][0] as i32;
        let dx = t[i][1] as i32 - t[i-1][1] as i32;
        let dy = t[i][2] as i32 - t[i-1][2] as i32;
        let d = dx.abs() + dy.abs();

        if d > dt {
            println!("{}", "No");
            return;
        }

        if dt % 2 == 0 {
            if d % 2 != 0 {
                println!("{}", "No");
                return;
            }
        } else {
            if d % 2 == 0 {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}
