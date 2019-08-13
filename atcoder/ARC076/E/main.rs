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
      r: usize,
      c: usize,
      n: usize,
      xys: [(usize,usize,usize,usize); n],
    }

    let mut points = vec![];

    let edge = |x, y| (x == r || x == 0) || (y==c || y==0);
    let to_pos = |x, y| {
        if x == 0 { return y }
        if y == c { return x + c }
        if x == r { return r + c + (c-y) }
        if y == 0 { return r + 2*c + (r-x) }
        unreachable!()
    };

    let mut num = 0;
    for (x1, y1, x2, y2) in xys {
        num += 1;
        if edge(x1, y1) && edge(x2, y2) {
            let pos1 = to_pos(x1, y1);
            let pos2 = to_pos(x2, y2);
            points.push((pos1, num));
            points.push((pos2, num));
        }
    }
    points.sort();

    // debug!(points);

    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    for (_, i) in points {
        if let Some(&v) = q.back() {
            if v == i {
                q.pop_back();
                continue;
            }
        }
        q.push_back(i);
    }
    if q.len() == 0 {
        puts!("{}\n", "YES");
    } else {
        puts!("{}\n", "NO");
    }
}
