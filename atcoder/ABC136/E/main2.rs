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
use std::io::{stdout, stdin, BufWriter, Write};

fn prime_factorization(mut n: usize) -> Vec<(usize, usize)> {
    let mut i = 2;
    let mut ans = vec![];
    while i*i <= n {
        let mut cnt = 0;
        while n%i == 0 {
            n /= i;
            cnt += 1;
        }
        if cnt > 0 {
            ans.push((i, cnt));
        }
        i += 1;
    }
    if n > 1 {
        ans.push((n, 1));
    }
    ans
}

fn rec(primes: &Vec<(usize,usize)>, cur: usize, value: usize, f: &mut FnMut(usize)) {
    if cur == primes.len() {
        f(value);
        return
    }
    let (p, num) = primes[cur];
    for i in 0..num+1 {
        let v = p.pow(i as u32);
        rec(primes, cur+1, value*v, f);
    }
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      k: usize,
      aa: [usize; n],
    }
    let sum: usize = aa.iter().sum();
    let primes = prime_factorization(sum);


    let mut ans = 1;
    {
        let mut f = |v| {
            let mut temp = vec![];
            for i in 0..n {
                let a = aa[i];
                let v1 = (a%v) as i64;
                let v2 = v as i64 - v1;
                if v2 < v1 {
                    temp.push(v2);
                } else {
                    temp.push(-1 * v1);
                }
            }
            temp.sort();
            let s: i64 = temp.iter().sum();
            let cnt = (s/v as i64).abs() as usize;
            if s > 0 {
                for i in 0..cnt {
                    temp[n-i-1] -= v as i64;
                }
            } else if s < 0 {
                for i in 0..cnt {
                    temp[i] += v as i64;
                }
            }
            // debug!(s, cnt, temp);

            let mut req = 0;
            for i in 0..n {
                if temp[i] > 0 {
                    req += temp[i];
                }
            }
            if req as usize <= k {
                ans = max(ans, v);
            }
        };
        rec(&primes, 0, 1, &mut f);
    }
    puts!("{}\n", ans);
}
