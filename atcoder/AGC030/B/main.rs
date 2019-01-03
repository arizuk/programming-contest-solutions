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
      l: usize,
      n: usize,
      xs: [usize; n],
    }
    let mut xs = xs;
    let mut sx = vec![0; n+1];
    let mut sy = vec![0; n+1];
    let mut ys: Vec<usize> = xs.iter().map(|v| l - *v).collect();
    ys.reverse();

    for i in 0..n {
        sx[i+1] = sx[i] + xs[i];
        sy[i+1] = sy[i] + ys[i];
    }
    // debug!(sx, sy);

    let mut ans = 0;
    for i in 0..n {
        let mut d = 0;
        let nx = (n-i-1)/2;
        let ny = (n-i)/2;

        d += sx[i+1+nx] - sx[i];
        d += sy[ny];
        d *= 2;

        if (n-i-1)%2==0 {
            d -= xs[i+nx];
        } else {
            d -= ys[ny-1];
        }
        ans = max(ans, d);
    }

    std::mem::swap(&mut xs, &mut ys);
    std::mem::swap(&mut sx, &mut sy);

    for i in 0..n {
        let mut d = 0;
        let nx = (n-i-1)/2;
        let ny = (n-i)/2;

        d += sx[i+1+nx] - sx[i];
        d += sy[ny];
        d *= 2;

        if (n-i-1)%2==0 {
            d -= xs[i+nx];
        } else {
            d -= ys[ny-1];
        }
        ans = max(ans, d);
    }
    println!("{}", ans);
}
