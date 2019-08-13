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

// KMP法
fn make_kmp_table(s: &Vec<char>) -> Vec<i64> {
    let n = s.len();
    let mut t = vec![0; n+1];
    t[0] = -1;

    let mut pos = 0;
    for i in 2..n+1 {
        let prev = s[i-1];
        if prev == s[pos] {
            pos += 1;
            t[i] = pos as i64;
        } else if pos > 0 {
            pos = t[pos] as usize;
        } else {
            pos = 0;
        }
    }
    t
}

fn calc_maximum_length(partial: &Vec<bool>, n: usize, m: usize) -> i64 {
    use std::collections::HashSet;
    let mut cnts = vec![-1; n];
    for i in 0..n {
        if cnts[i] >= 0 {
            continue;
        }
        let mut cur = i;
        let mut seen = HashSet::new();
        while partial[cur%n] {
            let v = cur%n;
            if seen.contains(&v) {
                return -1
            }
            seen.insert(v);
            cur += m;
        }
        let mut cnt = 0;
        while cur > i {
            cur -= m;
            cnt += 1;
            cnts[cur%n] = cnt;
        }
    }
    // debug!(cnts);
    let ans = max(*cnts.iter().max().unwrap(), 0);
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      s: chars,
      t: chars,
    }
    let n = s.len();
    let m = t.len();
    let tbl = make_kmp_table(&t);

    let mut s_idx = 0;
    let mut offset = 0;
    let mut partial = vec![false; n];
    while s_idx < n {
        let mut t_idx = offset;
        let mut ok = true;
        while t_idx < m {
            if s[(s_idx + t_idx) % n] != t[t_idx] {
                ok = false;
                break;
            }
            t_idx += 1;
        }
        if ok {
            partial[s_idx] = true;
        }
        s_idx = ((s_idx + t_idx) as i64 - tbl[t_idx]) as usize;
        offset = max(tbl[t_idx], 0) as usize;
    }

    let ans = calc_maximum_length(&partial, n, m);
    puts!("{}\n", ans);
}
