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
      aa: [usize; n],
      bb: [usize; n],
    }

    let mut aa2 = aa.clone();
    aa2.sort();
    let mut bb2 = bb.clone();
    bb2.sort();

    for i in 0..n {
        if aa2[i] > bb2[i] {
            return puts!("{}\n", "No");
        }
    }

    for i in 0..n {
        // 1 2 3 3
        // 1 2 3 4
        if i > 0 {
            if aa2[i] <= bb2[i-1] {
                return puts!("{}\n", "Yes");
            }
        }
    }


    // 愚直にシミュレーション
    let mut bb: Vec<_> = bb.into_iter().enumerate().map(|(i, v)| (v, i)).collect();

    let mut aa2 = vec![];
    for &(_, idx) in bb.iter() {
        aa2.push( aa[idx] );
    }

    let mut aa = aa2;
    let mut a_pos: Vec<_> = aa.iter().cloned().enumerate().map(|(i, v)| (v, i)).collect();
    a_pos.sort();
    let mut a_pos: Vec<_> = a_pos.iter().map(|v|v.1).collect();

    let mut sorted_a: Vec<_> = aa.clone();
    sorted_a.sort();

    for i in 0..n-1 {
        let b = bb[i].0;

        // swap i and pos;
        let pos = a_pos[i];

        debug!(aa, a_pos, i, pos);
        if i == pos {
            return puts!("{}\n", "Yes");
        }

        let a = aa[i];
        let sorted_index = sorted_a.binary_search(&a).unwrap();

        let temp = aa[i];
        aa[i] = aa[pos];
        aa[pos] = temp;

        a_pos[i] = i;
        a_pos[sorted_index] = pos;
    }

    puts!("{}\n", "No");
}
