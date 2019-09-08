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
      a: usize1,
      k_s: chars,
      bs: [usize1; n],
    }

    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut path = vec![];
    let mut cur = a;
    let mut ok = false;
    for i in 0..n {
        seen.insert(cur);
        path.push(cur);

        let nx = bs[cur];
        if seen.contains(&nx) {
            path.push(nx);
            ok = true;
            break;
        }
        cur = nx;
    }
    assert!(ok);
    // debug!(path, k);
    let mut s = 0;
    let mut last = path[path.len()-1];
    for i in 0..path.len() {
        if path[i] == last {
            s = i;
            break;
        }
    }

    let cycle = (&path[s..path.len()-1]).clone();
    let m = cycle.len();
    let mut k = 0;
    if k_s.len() <= 6 {
        for c in k_s {
            k *= 10;
            k += c.to_digit(10).unwrap() as usize;
        }
        if k <= s {
            return puts!("{}\n", path[k]+1);
        }
        let idx = ((k%m) + m - s) % m;
        return puts!("{}\n", cycle[idx]+1);
    }

    let mut k = 0;
    for c in k_s {
        k *= 10;
        k += c.to_digit(10).unwrap() as usize;
        k %= m;
    }
    let idx = ((k%m) + m - s) % m;
    return puts!("{}\n", cycle[idx]+1);
}
