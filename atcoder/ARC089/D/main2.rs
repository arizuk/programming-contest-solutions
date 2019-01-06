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
      n: usize,
      k: usize,
      xycs: [(usize, usize, String); n]
    }

    let mut xycs: Vec<(usize, usize, bool)> = xycs.iter().map(|v|
        (v.0%k, v.1%k, (if (v.0/k + v.1/k) % 2 == 0 { 'W' } else { 'B' }).to_string() == v.2)
    ).collect();

    let mut cnt = xycs.iter().fold(0, |acc, v| if v.2 { acc + 1 } else { acc });
    let mut xs: Vec<(usize, usize)> = xycs.iter().enumerate().map(|(i, v)| (v.0, i)).collect();
    xs.sort();
    let mut ys: Vec<(usize, usize)> = xycs.iter().enumerate().map(|(i, v)| (v.1, i)).collect();
    ys.sort();

    let mut ddy = 1i64;
    let mut dy = 0;
    let mut ans = max(cnt, n-cnt);
    let mut yi = n-1;
    let mut xi = n-1;
    let mut x_end = false;
    debug!(xycs);

    debug!(n, cnt);
    for dx in 0..k {
        // dx: 0 -> k
        if !x_end {
            while xs[xi].0 + dx >= k {
                xycs[xs[xi].1].2 = !xycs[xs[xi].1].2;
                if xycs[xs[xi].1].2 {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
                if xi == 0 {
                    x_end = true;
                    break;
                }
                xi -= 1;
            }
            ans = max(ans, max(cnt, n-cnt));
        }

        let mut last_yi = n;
        if yi < n {
            let mut y_end = false;
            loop {
                if ddy > 0 {
                    // dy: 0 -> k
                    while ys[yi].0 + dy >= k {
                        xycs[ys[yi].1].2 = !xycs[ys[yi].1].2;
                        if xycs[ys[yi].1].2 {
                            cnt += 1;
                        } else {
                            cnt -= 1;
                        }
                        last_yi = yi;
                        if yi == 0 {
                            y_end = true;
                            break;
                        }
                        yi -= 1;
                    }
                    if dy == k-1 {
                        y_end = true;
                    }

                } else {
                    // dy: k -> 0
                    while ys[yi].0 + dy < k {
                        xycs[ys[yi].1].2 = !xycs[ys[yi].1].2;
                        if xycs[ys[yi].1].2 {
                            cnt += 1;
                        } else {
                            cnt -= 1;
                        }
                        last_yi = yi;
                        if yi == n-1 {
                            y_end = true;
                            break;
                        }
                        yi += 1;
                    }

                    if dy == 0 {
                        y_end = true;
                    }
                }
                ans = max(ans, max(cnt, n-cnt));
                if y_end {
                    break;
                }
                dy = (dy as i64 + ddy) as usize;
            }
        }
        ddy *= -1;
        dy = if ddy > 0 { 0 } else { k-1 };
        yi = last_yi;
    }
    println!("{}", ans);
}
