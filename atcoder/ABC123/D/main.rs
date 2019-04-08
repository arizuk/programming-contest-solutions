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

use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
    input!{
      x: usize,
      y: usize,
      z: usize,
      k: usize,
      mut aa: [usize; x],
      mut bb: [usize; y],
      mut cc: [usize; z],
    }
    aa.sort();
    aa.reverse();
    bb.sort();
    bb.reverse();
    cc.sort();
    cc.reverse();

    let mut map = HashMap::new();
    let sum = |i, j, k| {
        return aa[i] + bb[j] + cc[k];
    };

    let new_entry = |i, j, k| {
        return (sum(i, j, k), i, j, k)
    };

    let mut heap = BinaryHeap::new();
    heap.push(new_entry(0, 0, 0));
    map.insert((0, 0, 0), true);

    let mut cnt = 0;
    while cnt < k {
        cnt += 1;
        let (sum, i, j, k) = heap.pop().unwrap();
        println!("{}", sum);

        let items = [
            (i+1, j, k),
            (i, j+1, k),
            (i, j, k+1),
        ];

        for &(i, j, k) in items.iter() {
            if !map.contains_key(&(i, j, k)) {
                if i < x && j < y && k < z {
                    let e = new_entry(i, j, k);
                    heap.push(e);
                    map.insert((i, j, k), true);
                }
            }
        }
    }
}
