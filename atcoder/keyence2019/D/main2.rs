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
use std::collections::HashSet;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    input!{
      n: usize,
      m: usize,
      aa: [usize; n],
      bb: [usize; m],
    }

    let au: HashSet<usize> = aa.iter().map(|&v| v).collect();
    if au.len() < n {
        return println!("{}", 0);
    }
    let bu: HashSet<usize> = bb.iter().map(|&v| v).collect();
    if bu.len() < m {
        return println!("{}", 0);
    }


    let mut ans = 1;
    let mut rbigger = 0;
    let mut cbigger = 0;
    for i in (1..(n*m)+1).rev() {
        let rmax = au.contains(&i);
        let cmax = bu.contains(&i);

        if rmax {
            rbigger += 1;
        }
        if cmax {
            cbigger += 1;
        }

        if rmax && cmax {
        } else if rmax {
            ans *= cbigger;
            ans %= MOD;
        } else if cmax {
            ans *= rbigger;
            ans %= MOD;
        } else {
            ans *= rbigger*cbigger - (n*m-i);
            ans %= MOD;
        }
    }
    println!("{}", ans);
}

