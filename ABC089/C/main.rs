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
use std::cmp::{min, max};

fn main() {
    input!{
      n: usize,
      ss: [chars; n],
    }
    let mut c = vec![0; 5];
    for i in 0..n {
        match ss[i][0] {
            'M' => c[0] += 1,
            'A' => c[1] += 1,
            'R' => c[2] += 1,
            'C' => c[3] += 1,
            'H' => c[4] += 1,
            _ => continue,
        }
    }

    let uc = c.iter().filter(|&&v| v >= 1).count() as u64;
    if uc < 3 {
        println!("{}", 0);
        return
    }
    let mut ans: u64 = 0;
    for i in 0..5 {
        for j in i+1..5 {
            for k in j+1..5 {
                ans += c[i] * c[j] * c[k];
            }
        }
    }
    println!("{}", ans);
}
