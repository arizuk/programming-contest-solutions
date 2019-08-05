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
      s: chars,
    }
    let n = s.len();

    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut r_table = vec![0; n];
    for i in 0..n {
        if s[i] == 'R' {
            q.push_back(i);
        } else {
            while let Some(idx) = q.pop_back() {
                r_table[idx] = i;
            }
        }
    }

    let mut q = VecDeque::new();
    let mut l_table = vec![0; n];
    for i in (0..n).rev() {
        if s[i] == 'L' {
            q.push_back(i);
        } else {
            while let Some(idx) = q.pop_back() {
                l_table[idx] = i;
            }
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let c = s[i];
        if c == 'R' {
            let l_idx = r_table[i];
            if (l_idx - i) % 2 == 0 {
                ans[l_idx] += 1;
            } else {
                ans[l_idx-1] += 1;
            }
        } else {
            let r_idx = l_table[i];
            if (i - r_idx) % 2 == 0 {
                ans[r_idx] += 1;
            } else {
                ans[r_idx+1] += 1;
            }
        }
    }
    for a in ans {
        puts!("{} ", a);
    }
    puts!("{}\n", "");
}