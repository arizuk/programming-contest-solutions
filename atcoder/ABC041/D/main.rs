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

fn fact(n: usize) -> usize {
    let mut ret = 1;
    for i in 1..n+1 {
        ret *= i;
    }
    ret
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      xys: [(usize, usize); m],
    }

    let mut e = vec![vec![]; n+2];
    let mut h = vec![0; n+2];
    for &(x, y) in xys.iter() {
        e[x].push(y);
        h[y] += 1;
    }

    for i in 1..n+1 {
        if e[i].len() == 0 {
            e[i].push(n+1);
            h[n+1] += 1;
        }
    }
    for i in 1..n+1 {
        if h[i] == 0 {
            e[0].push(i);
            h[i] += 1;
        }
    }

    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut vs = vec![];
    while let Some(i) = q.pop_front() {
        vs.push(i);
        for &nd in e[i].iter() {
            h[nd] -= 1;
            if h[nd] == 0 {
                q.push_back(nd);
            }
        }
    }

    let mut sums = vec![0; n+2];
    let mut temp = vec![vec![]; n+2];
    temp[0].push(1);
    let mut ans = 1usize;
    for v in vs {
        let p = 1;
        let mut carry = temp[v][0];
        for i in 1..temp[v].len() {
            let v2 = temp[v][i];
            let temp2 = fact(carry + v2 - 2) / fact(carry-1) / fact(v2-1);
            ans *= temp2;
        }

        let mut sum = 0;
        for &num in temp[v].iter() {
            sum += num;
        }
        if temp[v].len() > 0 {
            sum = sum - temp[v].len() + 1;
        }
        sums[v] = sum:
        for &nd in e[v].iter() {
            temp[nd].push(sum + 1);
        }
    }
    puts!("{}\n", ans);
}
