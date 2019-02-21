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
use std::collections::HashMap;

fn main() {
    input!{
      n: usize,
      aa: [usize; n*n],
    }
    let mut f1 = 0;
    let mut f2 = 0;
    let mut f4 = (n/2).pow(2);
    if n%2 == 1 {
        f1 = 1;
        f2 = n-1;
        f4 = ((n-1)/2).pow(2);
    }
    let mut map = HashMap::new();
    for a in aa {
        let e = map.entry(a).or_insert(0);
        *e += 1;
    }

    let mut table = vec![vec![0; n]; n];
    let h = n/2;
    let mut f4_idx = 0;
    let mut f2_idx = 0;

    for (&a, &cnt) in map.iter() {
        let mut cnt = cnt;
        while cnt > 0 {
            // debug!(f1, f2, f4, a, cnt, table);
            if f4 > 0 && cnt%4== 0 {
                let x = f4_idx/h;
                let y = f4_idx%h;
                table[x][y] = a;
                table[n-x-1][y] = a;
                table[x][n-y-1] = a;
                table[n-x-1][n-y-1] = a;

                f4 -= 1;
                f4_idx += 1;
                cnt -= 4;
                continue;
            }
            else if f2 > 0 && cnt%2 == 0 {
                if f2_idx < (n-1)/2 {
                    let x = h;
                    let y = f2_idx;
                    table[x][y] = a;
                    table[x][n-y-1] = a;
                } else {
                    let x = f2_idx%((n-1)/2);
                    let y = h;
                    table[x][y] = a;
                    table[n-x-1][y] = a;
                }
                f2 -= 1;
                f2_idx += 1;
                cnt -= 2;
                continue;
            }
            else if f1 > 0 && cnt%2 == 1 {
                table[n/2][n/2] = a;
                f1 -= 1;
                cnt -= 1;
                continue;
            }
            return println!("{}", "NO");
        }
    }

    println!("{}", "YES");
    for i in 0..n {
        for j in 0..n {
            print!("{} ", table[i][j]);
        }
        println!("");
    }
}
