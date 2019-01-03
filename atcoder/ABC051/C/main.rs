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
      sx: i64,
      sy: i64,
      tx: i64,
      ty: i64,
    }
    debug!(sx, sy, tx, ty);

    let mut dy = 1;
    while sy + dy <= ty {
        print!("{}", 'U');
        dy += 1;
    }

    let mut dx = 1;
    while sx + dx <= tx {
        print!("{}", 'R');
        dx += 1;
    }

    let mut dy = -1;
    while ty + dy >= sy {
        print!("{}", 'D');
        dy -= 1;
    }

    let mut dx = -1;
    while tx + dx >= sx {
        print!("{}", 'L');
        dx -= 1;
    }


    // 2
    print!("{}", 'L');
    let mut dy = 1;
    while sy + dy <= ty+1 {
        print!("{}", 'U');
        dy += 1;
    }

    let mut dx = 1;
    while sx - 1 + dx <= tx {
        print!("{}", 'R');
        dx += 1;
    }
    print!("{}", "DR");

    let mut dy = -1;
    while ty + dy >= sy-1 {
        print!("{}", 'D');
        dy -= 1;
    }

    let mut dx = -1;
    while tx + 1 + dx >= sx {
        print!("{}", 'L');
        dx -= 1;
    }
    print!("{}", 'U');
    println!();
}
