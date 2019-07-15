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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
    }
    if n <= 2 {
        return puts!("{}", "No");
    }
    let mut tmp = 1;
    while tmp*2 <= n {
        tmp *= 2;
    }
    if tmp == n {
        return puts!("{}", "No");
    }

    use std::collections::HashSet;
    let mut set = HashSet::new();

    let mut ans = vec![];
    ans.push((3, 1+n));
    for i in 1..(n-1)/2+1 {
        let a = i*2;
        let b = i*2+1;
        ans.push((1, a));
        ans.push((a, b));
        ans.push((1, b+n));
        ans.push((b+n, a+n));

        set.insert(a);
        set.insert(b+n);
    }

    if n%2 == 0 {
        let mut a = 1;
        while a*2 < n {
            a *= 2;
        }
        let b = n^a^1;

        if set.contains(&a) {
            ans.push((a, n));
        } else {
            ans.push((a+n, n));
        }

        if set.contains(&b) {
            ans.push((b, n+n));
        } else {
            ans.push((b+n, n+n));
        }
    }

    puts!("{}", "Yes");
    for a in ans {
        puts!("{} {}", a.0, a.1);
    }
}
