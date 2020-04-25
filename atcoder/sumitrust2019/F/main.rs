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
      t1: usize,
      t2: usize,
      a1: usize,
      a2: usize,
      b1: usize,
      b2: usize,
    }
    if t1*a1 + t2*a2 == t1*b1 + t2*b2 {
        return puts!("{}\n", "infinity");
    }

    let ta1 = t1*a1;
    let tb1 = t1*b1;
    let ta = ta1 + t2*a2;
    let tb = tb1 + t2*b2;

    if a1 > b1 {
        if ta > tb {
            return puts!("{}\n", 0);
        }


        let cnt = t1 * (a1-b1) / (tb-ta);
        if t1 * (a1-b1) % (tb-ta) == 0 {
            puts!("{}\n", cnt*2);
        } else {
            puts!("{}\n", cnt*2+1);
        }
    } else {
        if tb > ta {
            return puts!("{}\n", 0);
        }

        let cnt = t1 * (b1-a1) / (ta-tb);
        if t1 * (b1-a1) % (ta-tb) == 0 {
            puts!("{}\n", cnt*2);
        } else {
            puts!("{}\n", cnt*2+1);
        }
    }
}
