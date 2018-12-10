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

fn n_c(c: char) -> char {
    return match c {
        'D' => 'M',
        'M' => 'C',
        _ => panic!("")
    }
}

fn f(i: usize, c: char, di: usize, mi: usize, s: &Vec<char>, k: usize, n: usize, dp: &mut Vec<Vec<i64>>) -> usize {
    let ci = match c {
        'D' => 0,
        'M' => 1,
        _ => 2,
    };
    if dp[i][ci] != -1 {
        return dp[i][ci] as usize;
    }

    if i == n {
        return 0;
    }
    let mut ans = 0;
    if s[i] == c {
        if c == 'C' {
            ans += if i - di < k { 1 } else { 0 }
        } else {
            let di = if c == 'D' { i } else { di };
            let mi = if c == 'M' { i } else { mi };
            let nc = n_c(c);
            ans += f(i+1, nc, di, mi, s, k, n, dp);
        }
    }
    ans += f(i+1, c, di, mi, s, k, n, dp);
    dp[i][ci] = ans as i64;
    ans
}

fn main() {
    input!{
      n: usize,
      s: chars,
      q: usize,
      ks: [usize; q],
    }
    for k in ks {
      let mut dp = vec![vec![-1; 3]; 100000];
      println!("{}", f(0, 'D', 0, 0, &s, k, n, &mut dp));
    }
}
