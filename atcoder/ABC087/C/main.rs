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

#[allow(unused_imports)]
use std::cmp;

fn serach(aa: &[Vec<usize>], sum: &mut [Vec<usize>], i: usize, j: usize) {
    if i + 1 < 2 {
        if sum[i+1][j] < sum[i][j] + aa[i+1][j] {
            sum[i+1][j] = sum[i][j] + aa[i+1][j];
            serach(aa, sum, i+1, j);
        }
    }
    if j + 1 < aa[0].len() {
        if sum[i][j+1] < sum[i][j] + aa[i][j+1] {
            sum[i][j+1] = sum[i][j] + aa[i][j+1];
            serach(aa, sum, i, j+1);
        }
    }
}

fn main() {
    input!{
        n: usize,
        aa: [[usize; n]; 2],
    }

    let mut sum = vec![vec![0; n]; 2];
    sum[0][0] = aa[0][0];
    serach(&aa, &mut sum, 0, 0);
    println!("{}", sum[1][n-1]);
}