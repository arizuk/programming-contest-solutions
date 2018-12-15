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
use std::collections::HashMap;

fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }

    let mut pp: Vec<HashMap<usize, Vec<usize>>> = vec![];
    for i in 1..33 {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        let p = 2usize.pow(i);
        for j in 0..n {
            let a = aa[j];
            if a >= p {
                continue;
            }
            let r = a%p;
            let e = map.entry(r).or_insert(vec![]);
            e.push(j);
        }
        pp.push(map);
    }

    let mut pairs = vec![(0, 0); n];
    {
        for i in 0..n {
            pairs[i].0 = i;
        }
    }

    for i in 0..n {
        let a = aa[i];
        for j in 1..33 {
            let p = 2usize.pow(j as u32);
            if a >= p {
                continue;
            }
            let r = p - a%p;
            let map = &pp[j-1];
            if let Some(vs) = map.get(&r) {
                pairs[i].1 += vs.len();
                if r == a%p {
                    pairs[i].1 -= 1;
                }
            }
        }
    }

    let mut used = vec![false; n];
    pairs.sort_by_key(|v| v.1);

    let mut ans = 0;
    for i in 0..n {
        let pair = &pairs[i];
        if pair.1 == 0 {
            continue;
        }
        let v1 = pair.0;
        if used[v1] {
            continue;
        }

        let a = aa[v1];
        for j in 1..33 {
            let p = 2usize.pow(j as u32);
            if a >= p {
                continue;
            }
            let r = p - a%p;
            let mut map = &mut pp[j-1];
            if map.contains_key(&r) {
                let e = map.entry(r).or_insert(vec![]);
                while e.len() > 0 {
                    if let Some(v2) = e.pop() {
                        if used[v2] {
                            continue;
                        }
                        if v2 == v1 {
                            continue;
                        }

                        used[v1] = true;
                        used[v2] = true;
                        ans += 1;
                        break;
                    }
                }
            }
            if used[v1] {
                break;
            }
        }
    }
    println!("{}", ans);
}