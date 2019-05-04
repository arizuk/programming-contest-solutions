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
type I = usize;

fn can_block(dir: usize, h: isize, w: isize, mut x: isize, mut y: isize, ss: &Vec<char>, ts: &Vec<char>) -> bool {
    // debug!(dir, h, w, x, y);
    let n = ss.len();
    for i in 0..n {
        match dir {
            0 => {
                if ss[i] == 'U' { x -=1; }
                if ss[i] == 'R' { y +=1; }
            },
            1 => {
                if ss[i] == 'D' { x +=1; }
                if ss[i] == 'R' { y +=1; }
            },
            2 => {
                if ss[i] == 'D' { x +=1; }
                if ss[i] == 'L' { y -=1; }
            },
            3 => {
                if ss[i] == 'U' { x -=1; }
                if ss[i] == 'L' { y -=1; }
            },
            _ => unreachable!()
        };
        if !(x>=0 && x<h && y>=0 && y < w) {
            return false
        }

        match dir {
            0 => {
                if ts[i] == 'D' { x = min(x+1, h-1); }
                if ts[i] == 'L' { y = max(y-1, 0); }
            },
            1 => {
                if ts[i] == 'U' { x = max(x-1, 0); }
                if ts[i] == 'L' { y = max(y-1, 0); }
            },
            2 => {
                if ts[i] == 'U' { x = max(x-1, 0); }
                if ts[i] == 'R' { y = min(y+1, w-1); }
            },
            3 => {
                if ts[i] == 'D' { x = min(x+1, h-1); }
                if ts[i] == 'R' { y = min(y+1, w-1); }
            },
            _ => unreachable!()
        };

        if !(x>=0 && x<h && y>=0 && y < w) {
            return false
        }
    }
    true
}

fn main() {
    input!{
      h: usize,
      w: usize,
      n: usize,
      sr: isize,
      sc: isize,
      ss: chars,
      ts: chars,
    }

    for i in 0..4 {
        if !can_block(i, h as _, w as _, sr-1, sc-1, &ss, &ts) {
            return println!("{}", "NO");
        }
    }
    println!("{}", "YES")
}
