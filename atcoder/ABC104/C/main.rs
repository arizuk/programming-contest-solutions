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

fn search(pcs: &[Vec<usize>], i: usize, ss: usize, pp: usize, flags: &mut [bool], f: &mut FnMut(usize, usize, &[bool])) {
    if i == pcs.len() {
        f(pp, ss, flags);
        return;
    }
    let p = pcs[i][0];
    let c = pcs[i][1];
    let s = p * 100 * (i + 1) + c;
    flags[i] = true;
    search(pcs, i+1, s + ss, p + pp, flags, f);
    flags[i] = false;
    search(pcs, i+1, ss, pp, flags, f);
}

fn main() {
    input!{
        d: usize,
        g: usize,
        pcs: [[usize; 2]; d],
    }
    let mut ans = 100000000000;
    let mut flags = vec![false; d];
    search(&pcs, 0, 0, 0, &mut flags, &mut |p, s, flags| {
        if s >= g {
            ans = min(ans, p);
            // println!("p={} s={} g={} fgs={:?}", p, s, g, flags);
        } else {
            // 中途半端パターン
            let remain = g - s;
            let mut ok = false;
            let mut ii = 0;
            for i in (0..d).rev() {
                if !flags[i] {
                    if pcs[i][0] * (i + 1) * 100 >= remain {
                        ii = i;
                        ok = true;
                    }
                    break;
                }
            }

            if ok {
                let mut num = remain / (100 * (ii + 1));
                if remain % (100 * (ii + 1)) != 0 { num += 1 };
                ans = min(ans, p + num);
                // println!("p={} s={} g={} fgs={:?}", p+num, s+(100 * num * (ii + 1)), g, flags);
            }
        }
    });
    println!("{}", ans);
}
