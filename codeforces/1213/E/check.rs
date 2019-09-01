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
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    // input!{
    //   n: usize,
    //   s: chars,
    //   t: chars,
    // }

    let candidates = vec![
        ('a', 'b', 'c'),
        ('a', 'c', 'b'),
        ('b', 'a', 'c'),
        ('b', 'c', 'a'),
        ('c', 'a', 'b'),
        ('c', 'b', 'a'),
    ];

    for s1 in ['a', 'b', 'c'].iter() {
        for s2 in ['a', 'b', 'c'].iter() {
            for t1 in ['a', 'b', 'c'].iter() {
                for t2 in ['a', 'b', 'c'].iter() {
                    let s = [*s1, *s2];
                    let t = [*t1, *t2];

                    // abcabc
                    let mut ok = false;
                    for &(a, b, c) in candidates.iter() {
                        let list = vec![
                            [a, b],
                            [b, c],
                            [c, a],
                        ];
                        if !list.contains(&s) && !list.contains(&t) {
                            ok = true;
                            break;
                        }
                    }

                    // abab cc
                    // for &(a, b, c) in candidates.iter() {
                    //     let list = vec![
                    //         [a, b],
                    //         [b, a],
                    //         [b, c],
                    //         [c, c],
                    //     ];
                    //     if !list.contains(&s) && !list.contains(&t) {
                    //         ok = true;
                    //         break;
                    //     }
                    // }

                    // aa bb cc
                    for &(a, b, c) in candidates.iter() {
                        let list = vec![
                            [a, a],
                            [b, b],
                            [c, c],
                            [a, b],
                            [b, c],
                        ];
                        if !list.contains(&s) && !list.contains(&t) {
                            ok = true;
                            break;
                        }
                    }

                    if !ok {
                        debug!(s, t);
                    }
                }
            }
        }
    }
}
