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

fn calc_diff(s: &[char], t: &[char]) -> usize {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for &c in s.iter() {
        *map.entry(c).or_insert(0) += 1;
    }
    for &c in t.iter() {
        let e = map.entry(c).or_insert(0);
        if *e >= 1 {
            *e -= 1;
        }
    }

    let mut diff = 0;
    for (_, &v) in map.iter() {
        diff += v;
    }
    diff
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      mut k: usize,
      s: chars,
    }
    let mut t = s.clone();
    for i in 0..n {
        let mut swap_index = i;

        // Check if i and j is swappable
        for j in i+1..n {

            let temp = t[i];
            t[i] = t[j];
            t[j] = temp;

            let mut head_diff = 0;
            for k in 0..i+1 {
                if s[k] != t[k] {
                    head_diff += 1;
                }
            }
            let tail_diff = calc_diff(&s[i+1..], &t[i+1..]);

            t[j] = t[i];
            t[i] = temp;

            if head_diff + tail_diff > k {
                continue;
            }

            if t[j] < t[swap_index] {
                swap_index = j;
            }
        }

        let temp = t[i];
        t[i] = t[swap_index];
        t[swap_index] = temp;
    }
    puts!("{}", t.into_iter().collect::<String>());
}
