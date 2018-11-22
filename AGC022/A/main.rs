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

fn get_seen(s: &[usize]) -> Vec<bool> {
    let mut seen = vec![false; 26 + 1];
    seen[0] = true;
    for i in 0..s.len() {
        if s[i] == 0 { return seen; }
        seen[s[i]] = true;
    }
    seen
}

fn print_str(s: &[usize]) {
    for i in 0..s.len() {
        if s[i] == 0 { break; }
        print!("{}", (s[i] as u8 -1 + 'a' as u8) as char);
    }
    println!();
}

fn main() {
    input!{
        ss: chars
    }
    let mut s = vec![0; 26];
    for i in 0..ss.len() {
        let c = ss[i];
        // 1-26
        let c = c as usize - 'a' as usize + 1;
        s[i] = c;
    }

    // 1
    if ss.len() < 26 {
        let idx = s.iter().position(|&v|v==0).unwrap();
        let value = get_seen(&s).iter().position(|&v|v==false).unwrap();
        s[idx] = value;
        print_str(&s);
    } else {
        let mut pool = vec![];
        let mut idx = (s.len() - 2) as isize;
        pool.push(s[s.len()-1]);

        'outer: while idx >= 0 {
            for &v in &pool {
                if s[idx as usize] < v {
                    s[idx as usize] = v;
                    break 'outer;
                }
            }
            pool.push(s[idx as usize]);
            idx -= 1;
        }
        if idx == -1 {
            println!("{}", -1);
            return;
        }

        let idx = idx as usize;
        for i in idx+1..26 {
            s[i] = 0;
        }
        print_str(&s);
    }
}
