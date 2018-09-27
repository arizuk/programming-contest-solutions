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

const INF: usize = std::usize::MAX;

fn divide(a: &[usize]) -> (usize, usize, usize) {
    let mut t = 0;
    for i in 0..a.len() {
        t += a[i];
    }

    let mut min = INF;
    let mut d = 0;
    for i in 0..a.len() {
        d += a[i];
        // println!("i={} d={} a[i]={} t={}", i, d, a[i], t);

        let s = (d as isize - (t - d) as isize).abs() as usize;
        if s < min {
            min = s;
        } else {
            // println!("i={} d={} a[i]={} t={}", i, d, a[i], t);
            return (i-1, d - a[i], t - (d - a[i]));
        }
    }
    (0, a[0], 0)
}

fn main() {
    input!{
        n: usize,
        aa: [usize; n],
    }
    let (i, s1, s2) = divide(&aa);

    let mut divs = vec![
        (0, i+1, s1),
        (i+1, aa.len(), s2),
    ];

    for _ in 0..2 {
        println!("{:?}", divs);

        let mut max = 0;
        let mut mi = 0;
        for i in 0..divs.len() {
            if divs[i].1 - divs[i].0 > 1 && divs[i].2 > max  {
                max = divs[i].2;
                mi = i;
            }
        }
        let div = divs[mi];
        let (i, s1, s2) = divide(&aa[div.0..div.1]);
        divs.remove(mi);
        divs.insert(mi, (div.0, div.0+i+1, s1));
        divs.insert(mi+1, (div.0+i+1, div.1, s2));
    }

    let mut min = INF;
    let mut max = 0;
    for i in 0..4 {
        min = cmp::min(min, divs[i].2);
        max = cmp::max(max, divs[i].2);
    }
    println!("{:?}", divs);
    println!("{}", max - min);
}
