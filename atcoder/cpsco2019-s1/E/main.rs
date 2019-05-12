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
use std::ops::Bound::Included;

fn main() {
    input!{
      n: usize,
      q: usize,
      aa: [usize; n],
      lrxs: [(usize, usize, usize); q],
    }
    use std::collections::BTreeSet;
    use std::collections::HashMap;
    let mut set = BTreeSet::new();
    let mut map = HashMap::new();
    for a in aa {
        set.insert(a);
        *map.entry(a).or_insert(a) += 1;
    }

    for i in 0..q {
        let (l, r, x) = lrxs[i];
        let mut xor = 0;
        let mut keys = vec![];
        let mut new_num = 0;

        for &key in set.range(Included(&l), Included(&r)) {
            let value = map.get(&key).unwrap();

            if value%2 != 0 {
                xor ^= key;
            }

            keys.push(key);
            new_num += *value;
        }

        for key in keys {
            set.remove(&key);
            map.remove(&key);
        }

        set.insert(x);
        *map.entry(x).or_insert(0) += new_num;
        println!("{}", xor);
    }
}
