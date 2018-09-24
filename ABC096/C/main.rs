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

fn main() {
    input!{
        h: usize,
        w: usize,
        ss: [chars; h],
    }

    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '#' {
                let mut ok = false;
                for &(dx, dy) in dirs.iter() {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;
                    if ni >= 0 && ni < h as i32 && nj >= 0 && nj < w as i32 {
                        if ss[ni as usize][nj as usize] == '#' {
                            ok = true;
                            break;
                        }
                    }
                }
                if !ok {
                    println!("{}", "No");
                    return;
                }
            }
        }
    }
    println!("{}", "Yes");
}
