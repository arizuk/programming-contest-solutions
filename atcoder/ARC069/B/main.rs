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

fn check(c: char, a2: char, a1: char, a3: char) -> bool {
    return match (c, a2) {
        ('o', 'S') => {
            a1 == a3
        },
        ('o', 'W') => {
            a1 != a3
        },
        ('x', 'S') =>{
            a1 != a3
        },
        ('x', 'W') => {
            a1 == a3
        },
        _ => unreachable!(),
    };
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      s: chars,
    }

    let table = vec![
        ('S', 'S'),
        ('S', 'W'),
        ('W', 'S'),
        ('W', 'W'),
    ];

    let inv = |c| if c == 'W' { 'S' } else { 'W' };

    for (a1, a2) in table {
        let mut ans = vec![];
        ans.push(a1);
        ans.push(a2);
        for i in 2..n {
            if s[i-1] == 'o' {
                let tmp = ans[i-2];
                if ans[i-1] == 'S' {
                    ans.push(tmp);
                } else {
                    ans.push(inv(tmp));
                }
            } else if s[i-1] == 'x' {
                let tmp = ans[i-2];
                if ans[i-1] == 'S' {
                    ans.push(inv(tmp));
                } else {
                    ans.push(tmp);
                }
            } else {
                // debug!(a1, a2, s[i-1]);
                unreachable!();
            }
        }

        let ok = check(s[n-1], ans[n-1], ans[n-2], ans[0])
            && check(s[0], ans[0], ans[n-1], ans[1]);
        if ok {
            let ans = ans.into_iter().collect::<String>();
            return puts!("{}", ans);
        }
    }
    puts!("{}", -1);
}
