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

fn main() {
    input!{
      h: usize,
      w: usize,
      mut ss: [chars; h]
    }
    let mut center = None;
    let mut cnt = 0;
    for i in 1..h-1 {
        for j in 1..w-1 {
            if ss[i][j] == '*' {
                if ss[i-1][j] == '*' && ss[i][j-1] == '*' && ss[i+1][j] == '*' && ss[i][j+1] == '*' {
                    center = Some((i, j));
                }
            }
        }
    }
    if center.is_none() {
        return println!("{}", "NO");
    }
    let center = center.unwrap();

    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '*' {
                cnt += 1;
            }
        }
    }

    let (x, y) = center;
    let mut cnt2 = 1;
    for i in (0..x).rev() {
        if ss[i][y] == '*' {
            cnt2 += 1;
        } else {
            break;
        }
    }

    for i in x+1..h {
        if ss[i][y] == '*' {
            cnt2 += 1;
        } else {
            break;
        }
    }

    for j in (0..y).rev() {
        if ss[x][j] == '*' {
            cnt2 += 1;
        } else {
            break;
        }
    }

    for j in y+1..w {
        if ss[x][j] == '*' {
            cnt2 += 1;
        } else {
            break;
        }
    }

    if cnt2 == cnt {
        println!("{}", "YES");
    } else {
        println!("{}", "NO");
    }
}
