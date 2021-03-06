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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      aa: [u64; n],
    }

    let mut aa2 = aa.clone();
    aa2.sort();
    aa2.dedup();
    if aa2.len() == 1 && aa2[0] == 0 {
        return puts!("{}", "Yes");
    }
    if aa2.len() > 3 {
        return puts!("{}", "No");
    }

    let mut a1 = 0;
    let mut a2 = 0;
    let mut a3 = 0;

    for &a in aa.iter() {
        if a == aa2[0] {
            a1 += 1;
        } else if a == aa2[1] {
            a2 += 1;
        } else {
            a3 += 1;
        }
    }

    if a3 == 0 {
        if a1 > a2 {
            if a1 == a2*2 && aa2[1] == 0 {
                puts!("{}", "Yes");
            } else {
                puts!("{}", "No");
            }
        } else {
            if a2 == a1*2 && aa2[0] == 0 {
                puts!("{}", "Yes");
            } else {
                puts!("{}", "No");
            }
        }
    } else {
        if !(a1 == a2 && a2 == a3) {
            return puts!("{}", "No");
        }

        if aa2[0]^aa2[1]^aa2[2] == 0 {
            return puts!("{}", "Yes");
        } else {
            return puts!("{}", "No");
        }
    }
}
