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
      h: usize,
      w: usize,
      k: usize,
      ss: [chars; h],
    }

    let mut ans = vec![vec![0; w]; h];
    let mut cur = 0;
    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '#' {
                cur += 1;
                ans[i][j] = cur;
                for k in (0..j).rev() {
                    if ans[i][k] != 0 {
                        break;
                    }
                    ans[i][k] = ans[i][j];
                }
            }
        }

        for j in 1..w {
            if ans[i][j] == 0 {
                ans[i][j] = ans[i][j-1];
            }
        }
    }

    for i in 0..h-1 {
        for j in 0..w {
            if ans[i+1][j] == 0 {
                ans[i+1][j] = ans[i][j];
            }
        }
    }

    for i in (1..h).rev() {
        for j in 0..w {
            if ans[i-1][j] == 0 {
                ans[i-1][j] = ans[i][j];
            }
        }
    }

    for a in ans {
        for i in 0..a.len() {
            if i == a.len() - 1 {
                puts!("{}", a[i]);
            } else {
                puts!("{} ", a[i]);
            }
        }
        puts!("\n")
    }
}
