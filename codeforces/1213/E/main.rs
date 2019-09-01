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

    input!{
      n: usize,
      s: chars,
      t: chars,
    }

    let candidates = vec![
        ('a', 'b', 'c'),
        ('a', 'c', 'b'),
        ('b', 'a', 'c'),
        ('b', 'c', 'a'),
        ('c', 'a', 'b'),
        ('c', 'b', 'a'),
    ];

    // abcabc
    let mut ok = false;
    for &(a, b, c) in candidates.iter() {
        let list = vec![
            vec![a, b],
            vec![b, c],
            vec![c, a],
        ];
        if !list.contains(&s) && !list.contains(&t) {
            puts!("{}\n", "YES");
            let mut temp = vec![a, b, c];
            let mut ans = vec![];
            for i in 0..3*n {
                ans.push( temp[i%3] );
            }
            let s: String = ans.iter().collect();
            puts!("{}\n", s);
            return;
        }
    }

    // abab cc
    for &(a, b, c) in candidates.iter() {
        let list = vec![
            vec![a, b],
            vec![b, a],
            vec![b, c],
            vec![c, c],
        ];
        if !list.contains(&s) && !list.contains(&t) {
            puts!("{}\n", "YES");
            let mut temp = vec![a, b];
            let mut ans = vec![];
            for i in 0..2*n {
                ans.push( temp[i%2] );
            }
            for i in 0..n {
                ans.push( c );
            }
            let s: String = ans.iter().collect();
            puts!("{}\n", s);
            return
        }
    }


    // aa bb cc
    for &(a, b, c) in candidates.iter() {
        let list = vec![
            vec![a, a],
            vec![a, b],
            vec![b, b],
            vec![b, c],
            vec![c, c],
        ];
        if !list.contains(&s) && !list.contains(&t) {
            puts!("{}\n", "YES");
            let mut ans = vec![];
            for i in 0..n { ans.push(a); }
            for i in 0..n { ans.push(b); }
            for i in 0..n { ans.push(c); }
            let s: String = ans.iter().collect();
            puts!("{}\n", s);
            return
        }
    }
    unreachable!()
}