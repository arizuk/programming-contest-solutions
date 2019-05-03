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
type I = usize;

fn char_code(c: &char) -> usize {
    (*c as u8 - 'a' as u8) as usize
}

fn solve(ss: &mut Vec<char>) -> Vec<char> {
    ss.sort();
    let mut count = vec![0; 27];
    let mut chars = vec![];
    for i in 0..ss.len() {
        let d = char_code(&ss[i]);
        count[d] += 1;
        if count[d] == 1 {
            chars.push(ss[i]);
        }
    }
    let mut ret = vec!['a'; chars.len()];
    let n = chars.len();

    if n == 3 {
        let d1 = char_code(&chars[0]) as isize;
        let d2 = char_code(&chars[1]) as isize;
        let d3 = char_code(&chars[2]) as isize;

        if (d1-d2).abs() == 1 {
            ret[0] = (d1 as u8 + 'a' as u8) as char;
            ret[1] = (d3 as u8 + 'a' as u8) as char;
            ret[2] = (d2 as u8 + 'a' as u8) as char;
        } else if (d2-d3).abs() == 1 {
            ret[0] = (d2 as u8 + 'a' as u8) as char;
            ret[1] = (d1 as u8 + 'a' as u8) as char;
            ret[2] = (d3 as u8 + 'a' as u8) as char;
        } else {
            ret[0] = (d1 as u8 + 'a' as u8) as char;
            ret[1] = (d2 as u8 + 'a' as u8) as char;
            ret[2] = (d3 as u8 + 'a' as u8) as char;
        }
    } else {
        if n%2 == 1 {
            ret[n/2] = chars[n/2];
        }

        let mut j = 0;
        for i in (0..n/2).rev() {
            let c1 = chars[i];
            let c2 = chars[n-i-1];
            if i%2 == 0 {
                ret[j] = c1;
                ret[n-j-1] = c2;
            } else {
                ret[j] = c2;
                ret[n-j-1] = c1;
            }
            j += 1;
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        let d = char_code(&ret[i]);
        for _ in 0..count[d] {
            ans.push(ret[i]);
        }
    }
    ans
}

fn check(ss: &Vec<char>) -> bool {
    for i in 1..ss.len() {
        let a1 = char_code(&ss[i]) as isize;
        let a2 = char_code(&ss[i-1])as isize;
        if (a1-a2).abs() == 1 {
            return false;
        }
    }
    true
}


fn main() {
    input!{
      t: usize,
      ss: [chars; t],
    }
    for i in 0..t {
        let mut s = ss[i].clone();
        let ans = solve(&mut s);
        if check(&ans) {
            println!("{}", ans.iter().collect::<String>());
        } else {
            println!("{}", "No answer");
        }
    }
}
