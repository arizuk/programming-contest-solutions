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

fn solve(n: i64) -> Vec<Vec<i64>> {
    let mut values = vec![];
    for i in 1..n {
        let v1 = (i - (i * n) % n) + i;
        let v2 = i - (i * (n-1)) % n;
        // debug!(v1, v2);
        if v1 >= 0 && v1 < n {
            values.push(v1);
        } else {
            values.push(v2);
        }
    }
    values.push(n);
    debug!(values);

    let mut ans = vec![];
    for i in 1..n+1 {
        let mut cnt = 1;
        let mut rows = vec![];
        for j in 1..n+1 {
            if i == j {
                rows.push(values[(i-1) as usize]);
            } else {
                rows.push(i + n*cnt);
                cnt += 1;
            }
        }
        ans.push(rows);
    }
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: i64,
    }
    if n % 2 == 0 {
        return puts!("{}\n", "No");
    }
    puts!("{}\n", "Yes");

    let mut ans = solve(n);

    for a in ans.iter() {
        for i in 0..a.len() {
            if i == a.len() - 1 {
                puts!("{}", a[i]);
            } else {
                puts!("{} ", a[i]);
            }
        }
        puts!("\n")
    }

    for i in 0..n as usize {
        let row: i64 = (0..n as usize).map(|j| ans[i][j]).sum();
        let col: i64 = (0..n as usize).map(|j| ans[j][i]).sum();

        if row%n != (i as i64 + 1) %n {
            panic!();
        }
        if col%n != (i as i64 + 1) %n {
            panic!();
        }
    }

}
