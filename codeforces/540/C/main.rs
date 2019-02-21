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
use std::io::Write;
use std::collections::HashMap;
use std::collections::BinaryHeap;

type Pair = (usize, usize);

fn put(pos: &[Pair], heap: &mut BinaryHeap<Pair>, tbl: &mut [Vec<usize>]) {
    let mut pairs = heap.pop().unwrap();
    if pairs.0 < pos.len() {
        println!("{}", "NO");
        std::process::exit(0);
    }

    for &(x, y) in pos.iter() {
        tbl[x][y] = pairs.1;
    }

    pairs.0 -= pos.len();
    heap.push(pairs);
}

fn main() {
    input!{
      n: usize,
      aa: [usize; n*n],
    }
    let mut map = HashMap::new();
    for a in aa {
        let e = map.entry(a).or_insert(0);
        *e += 1 ;
    }
    let mut heap = BinaryHeap::new();
    for (&a, &cnt) in map.iter() {
        heap.push((cnt, a));
    }
    let mut table = vec![vec![0; n]; n];
    let rev = |i| {
        n-i-1
    };

    let h = n/2;
    for i in 0..h {
        for j in 0..h {
            put(&[
                (i, j),
                (rev(i), j),
                (i, rev(j)),
                (rev(i), rev(j)),
            ], &mut heap, &mut table);
        }
    }

    if n%2 == 1 {
        for i in 0..h {
            put(&[ (i, h), (rev(i), h), ], &mut heap, &mut table);
        }
        for i in 0..h {
            put(&[ (h, i), (h, rev(i)), ], &mut heap, &mut table);
        }
        put(&[(h, h)], &mut heap, &mut table);
    }

    println!("{}", "YES");
    for i in 0..n {
        for j in 0..n {
            print!("{} ", table[i][j]);
        }
        println!();
    }
}
