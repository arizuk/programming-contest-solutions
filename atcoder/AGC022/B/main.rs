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

pub fn prime_factorization(mut n: usize) -> Vec<(usize, usize)> {
    let mut i = 2;
    let mut ans = vec![];
    while i * i <= n {
        let mut cnt = 0;
        while n % i == 0 {
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

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn check(a: &Vec<usize>) -> bool {
    let sum: usize = a.iter().sum();

    for &v in a.iter() {
        if v > 30000 {
            return false;
        }
        let v2 = sum -v;
        if gcd(v, v2) == 1 {
            return false;
        }
    }

    let mut g = a[0];
    for &v in a.iter() {
        g = gcd(v, g);
    }
    g == 1
}

fn solve(n: usize) -> Vec<usize> {
    if n == 3 {
        return vec![2, 5, 63];
    }

    let mut ans = vec![];
    let mut sum = 0;

    let mut num2 = min(n-2, 15000);
    let mut num3 = n - num2;
    if num3%2 == 1 {
        num3 += 1;
        num2 -= 1;
    }

    for i in 0..num2 {
        let v = i * 2 + 2;
        ans.push(v);
        sum += v;
    }
    if sum%3 != 0 {
        ans[num2-1] += 6 - sum%6;
    }

    for i in 0..num3 {
        ans.push(3*(2*i + 1));
    }

    if !check(&ans) {
        panic!();
    }
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
    }
    let ans = solve(n);
    for i in 0..ans.len() {
        if i == ans.len() - 1 {
            puts!("{}", ans[i]);
        } else {
            puts!("{} ", ans[i]);
        }
    }
    puts!("\n")
}
