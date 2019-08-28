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

fn rev(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 0 {
        ret *= 10;
        ret += n%10;
        n /= 10;
    }
    ret
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }



    input!{
      n: usize,
      m: usize,
    }

    use std::collections::HashSet;
    let mut ans = 0;

    let mut ok = HashSet::new();
    let mut ng = HashSet::new();

    for i in 1..n+1 {
        for j in 1..m+1 {
            let mut seen = HashSet::new();
            let mut x = i;
            let mut y = j;

            let mut inf = false;
            while x != 0 && y != 0 {
                if ok.contains(&(x, y)) {
                    inf = true;
                    break;
                }
                if ng.contains(&(x, y)) {
                    break;
                }
                if seen.contains(&(x, y)) {
                    inf = true;
                    break;
                }
                seen.insert((x, y));

                if x < y {
                    x = rev(x);
                } else {
                    y = rev(y);
                }

                if x < y {
                    y = y - x;
                } else {
                    x = x - y;
                }
            }

            if inf {
                ans += 1;
                for v in seen {
                    ok.insert(v);
                }
            } else {
                for v in seen {
                    ng.insert(v);
                }
            }
        }
    }
    puts!("{}\n", ans);
}
