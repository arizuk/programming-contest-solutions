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
    let n64 = n as u64;
    const MAX: usize = 2 * 100000 + 2;
    let mut prev = vec![None; MAX];
    let mut first = vec![None; MAX];
    let mut jump_to = vec![vec![0u64; n]; 40];
    for i in 0..n {
        if let Some(idx) = first[aa[i]] {
            jump_to[0][i] = (idx+n+1) as u64;
        } else {
            jump_to[0][i] = (i+n+1) as u64;
        }

        if let Some(prev_idx) = prev[aa[i]] {
            jump_to[0][prev_idx] = (i+1) as u64;
        }
        prev[aa[i]] = Some(i);
        if first[aa[i]].is_none() {
            first[aa[i]] = Some(i);
        }
    }
    debug!(aa);
    debug!(jump_to[0]);

    for j in 1..40 {
        for i in 0..n {
            let idx = jump_to[j-1][i] as usize %n;
            let n = n as u64;
            jump_to[j][i] = jump_to[j-1][idx] + jump_to[j-1][i]/n*n;
        }
    }

    let last = n as u64 * k;
    let mut cur = 0;
    loop {
        for j in (0..40).rev() {
            let pos = (cur % (n as u64)) as usize;
            let next_pos = jump_to[j][pos] + cur/n64*n64;
            if next_pos <= last {
                cur = next_pos;
            }
        }
        if last - cur <= n as u64 {
            break;
        }
    }

    let mut ans = vec![];
    while cur < last {
        let pos = (cur%n64) as usize;
        if jump_to[0][pos] <= n64 {
            let next_pos = jump_to[0][pos] + cur/n64*n64;
            assert!(next_pos > cur);
            cur = next_pos;
        } else {
            ans.push(aa[pos]);
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
    puts!("{}\n", "");
}
