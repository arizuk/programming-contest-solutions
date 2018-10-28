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
    input!(n: usize);
    let mut p = 0;
    for i in 1.. {
        if i * (i - 1) / 2 == n {
            p = i;
            break;
        }
        if i * (i - 1) / 2 > n {
            break;
        }
    }
    if p == 0 {
        println!("No");
        return;
    }

    let mut groups = vec![];
    for i in 0..p {
        for j in (i + 1)..p {
            groups.push((i, j));
        }
    }
    println!("groups={:?}", groups);

    assert_eq!(groups.len(), n);
    let mut ans = vec![vec![]; p];
    for (i, &(x, y)) in groups.iter().enumerate() {
        ans[x].push(i);
        ans[y].push(i);
    }
    println!("ans={:?}", ans);
    return

    println!("Yes");
    println!("{}", ans.len());
    for ans in ans.iter() {
        print!("{}", ans.len());
        for &ans in ans.iter() {
            print!(" {}", ans + 1);
        }
        println!();
    }
}