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
    let mut t = vec![];
    let mut cur = 0;
    for i in 0..s.len() {
        if s[i] == '1' {
            break;
        }
        t.push(s[i]);
        cur = i + 1;
    }

    while cur < n {
        let one_start = cur;

        // 1のzoneを飛ばす
        while !(cur == n || s[cur] == '0') {
            cur += 1;
        }
        if cur == n {
            for _ in one_start..n {
                t.push('0');
            }
            break;
        }

        // 0の始まり
        let zero_start = cur;
        while !(cur == n || s[cur] == '1') {
            cur += 1;
        }

        let one_num = zero_start - one_start;
        let zero_num = cur - zero_start;

        if one_num > zero_num {
            for _ in 0..one_num-zero_num {
                t.push('0');
            }
            for _ in 0..zero_num {
                t.push('1');
            }
        } else {
            for _ in 0..one_num {
                t.push('1');
            }
        }
        for _ in 0..zero_num {
            t.push('0');
        }
    }

    let s: String = t.into_iter().collect();
    puts!("{}\n", s);
}
