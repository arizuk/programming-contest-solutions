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
use std::io::{stdout, stdin, BufWriter, Write};

const MAX_D: usize = 20;

fn calc(a: &Vec<usize>, pos: usize, under: bool, ng: bool) -> usize {
    if pos == a.len() {
        return if ng {
            1
        } else {
            0
        }
    }

    let mut ans = 0;
    match (under, ng) {
        (true, true) => {
            ans += calc(a, pos+1, under, ng) * 10;
        },
        (true, false) => {
            ans += calc(a, pos+1, under, true) * 2;
            ans += calc(a, pos+1, under, ng) * 8;
        },
        (false, true) => {
            let digit = a[pos];
            ans += calc(a, pos+1, true, true) * digit;
            ans += calc(a, pos+1, false, true);
        },
        (false, false) => {
            let digit = a[pos];
            let mut ng_nums = 0;
            for i in 0..digit {
                if i == 4 || i == 9 {
                    ng_nums += 1;
                }
            }
            ans += calc(a, pos+1, true, true) * ng_nums;
            ans += calc(a, pos+1, true, false) * (digit - ng_nums);
            ans += calc(a, pos+1, false, digit==4 || digit==9);
        },
    }
    ans
}


fn to_vec(n: usize) -> Vec<usize> {
    let mut s: Vec<_> = n.to_string().chars().collect();
    s.reverse();
    let mut a = vec![0; MAX_D];
    let mut cur = a.len()-1;
    for c in s {
        a[cur] = c.to_digit(10).unwrap() as usize;
        cur -= 1;
    }
    a
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      a: usize,
      b: usize,
    }
    let v1 = calc(&to_vec(b), 0, false, false);
    let v2 = calc(&to_vec(a-1), 0, false, false);
    debug!(v1, v2);
    puts!("{}\n", v1-v2);
}
