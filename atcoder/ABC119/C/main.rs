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

fn search(ls: &[i64], ans: &mut i64, i: usize, n: usize, vals: &mut [usize], abc: &[i64]) {
    if i == n {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut mp = 0;
        let mut cnt = vec![0i64; 4];
        for i in 0..vals.len() {
            let v = vals[i];
            let l = ls[i];

            if v != 0 {
                cnt[v] += 1;
            }
            if v == 1 {
                a += l;
            }
            if v == 2 {
                b += l;
            }
            if v == 3 {
                c += l;
            }
        }
        for i in 1..4 {
            if cnt[i] == 0 {
                return;
            }
            if cnt[i] >= 1 {
                mp += (cnt[i]-1) * 10;
            }
        }
        mp += (abc[0]-a as i64).abs();
        mp += (abc[1]-b as i64).abs();
        mp += (abc[2]-c as i64).abs();

        if mp < *ans {
            *ans = mp;
        }
        return;
    }

    for j in 0..4 {
        vals[i] = j;
        search(ls, ans, i+1, n, vals, abc);
        vals[i] = 0;
    }
}

fn main() {
    input!{
      n: usize,
      a: i64,
      b: i64,
      c: i64,
      ls: [i64; n]
    }
    let mut ans = 1 << 50;
    let mut vals = vec![0; n];
    search(&ls, &mut ans, 0, n, &mut vals, &[a, b, c]);
    println!("{}", ans);
}
