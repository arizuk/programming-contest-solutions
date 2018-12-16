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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn main() {
    input!{
      n: usize,
      aa: [usize; n]
    }
    let mut aa = aa;
    aa.reverse();

    if n == 1 {
        println!("{}", 1);
        return;
    }

    let mut ans = 1;
    let mut chs = vec![(1, aa[0])];
    // debug!(chs);
    for i in 0..n-1 {
        let a1 = aa[i];
        let a2 = aa[i+1];
        if a1 <= a2 {
            let mut insert_idx = chs.len();
            for i in (0..chs.len()).rev() {
                let ch = chs[i];
                if ch.0 < ans {
                    insert_idx = i;
                    break;
                }
            }

            if insert_idx < chs.len() {
                chs.resize(insert_idx+1, (0, 0));
                let mut tmp = chs.pop().unwrap();
                tmp.1 -= 1;
                if tmp.1 > 0 {
                    chs.push(tmp);
                }
                chs.push((tmp.0+1, 1));

                // merge
                if chs.len() >= 2 {
                    if chs[chs.len()-1].0 == chs[chs.len()-2].0 {
                        let tmp = chs.pop().unwrap();
                        let last = chs.len()-1;
                        chs[last].1 += tmp.1;
                    }
                }

                let sum = chs.iter().map(|v| v.1).sum::<usize>();
                if a2-sum > 0 {
                    chs.push((1, a2-sum));
                }
            } else {
                let mut tmp = chs.pop().unwrap();
                tmp.1 -= 1;
                if tmp.1 > 0 {
                    chs.push(tmp);
                }
                ans += 1;
                chs.push((ans, 1));

                // merge
                if chs.len() >= 2 {
                    if chs[chs.len()-1].0 == chs[chs.len()-2].0 {
                        let tmp = chs.pop().unwrap();
                        let last = chs.len()-1;
                        chs[last].1 += tmp.1;
                    }
                }

                if a2-a1 > 0 {
                    chs.push((1, a2-a1));
                }
            }
        } else {
            let mut t = a1 - a2;
            loop {
                if let Some(mut tmp) = chs.pop() {
                    if tmp.1 < t {
                        t -= tmp.1;
                        continue;
                    } else {
                        tmp.1 -= t;
                        if tmp.1 > 0 {
                            chs.push(tmp);
                        }
                        break;
                    }
                }
            }
        }
        // debug!(i, chs);
    }
    println!("chs={:?}", chs);
    println!("{}", ans);
}
