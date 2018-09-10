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

use std::collections::BTreeSet;
fn main() {
    input!{
        n: usize,
        w: [chars; n]
    }

    let mut ans = "Yes";
    for i in 0..(n-1) {
        if w[i][w[i].len() - 1] != w[i + 1][0] {
            ans = "No";
            break;
        }
    }

    let set = w.iter().collect::<BTreeSet<_>>();
    if set.len() != n { ans = "No"; }
    // println!("{:?}", set);
    // let a = vec!['h', 'o', 'g', 'e'];
    // println!("{}", set.contains(&a));

    // let mut set = std::collections::BTreeSet::new();
    // for w in w.iter() {
    //     if set.contains(w) {
    //         ans = "No";
    //     }
    //     set.insert(w);
    // }
    println!("{}", ans);
}