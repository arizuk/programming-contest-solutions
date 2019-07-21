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
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      k: u64,
      aa: [usize; n],
    }

    const MAX: usize = 2 * 100000 + 2;
    let mut next_n = vec![None; n];
    let mut last_n = vec![None; MAX];
    let mut first_n = vec![n; MAX];
    for i in 0..n {
        let a = aa[i];
        if let Some(idx) = last_n[a] {
            next_n[idx] = Some(i);
        }
        last_n[a] = Some(i);
        if first_n[a] == n {
            first_n[a] = i;
        }
    }

    let mut memo = vec![None; n];

    let mut cnt = 0;
    let mut left = None;
    loop {
        cnt += 1;
        let mut start_pos = 0usize;
        if let Some(l) = left {
            start_pos = first_n[l] + 1;
        }

        if start_pos == n {
            break;
        }

        let mut cur = start_pos;
        while cur < n {
            if let Some(v) = memo[cur] {
                memo[start_pos] = Some(v);
                break;
            }

            let a = aa[cur];
            if let Some(idx) = next_n[cur] {
                cur = idx + 1;
            } else {
                // fix next number
                memo[start_pos] = Some(a);
                break;
            }
        }

        if let Some(v) = memo[start_pos] {
            left = Some(v);
        } else {
            break;
        }
    }
    let k = k%cnt;
    if k == 0 {
        return puts!("{}\n", "");
    }

    let mut start_pos = 0;
    for i in 0..k-1 {
        let nv = memo[start_pos].unwrap();
        start_pos = first_n[nv] + 1;
    }

    // 最終的な文字列を求める
    let mut cur = start_pos;
    let mut ans = vec![];
    while cur < n {
        if next_n[cur].is_some() {
            cur = next_n[cur].unwrap() + 1;
            continue;
        } else {
            ans.push(aa[cur]);
            cur += 1;
        }
    }
    for &a in ans.iter() {
        if a == ans[ans.len()-1] {
            puts!("{}", a);
        } else {
            puts!("{} ", a);
        }
    }
    puts!("\n");
}
