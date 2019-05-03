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
use std::collections::HashMap;
type I = usize;

fn get_dup(aa: &[usize]) -> usize {
    let mut count = [0; 5];
    for &a in aa.iter() {
        count[a] += 1;
    }
    for i in 0..5 {
        if count[i] >= 2 {
            return i
        }
    }
    5
}


fn rec(ranks: &[Vec<usize>], indices: [usize; 4], idx: usize, n: usize, ans: &mut Vec<usize>) -> bool {
    if idx == n {
        return true;
    }

    let mut temp = vec![];
    for i in 0..4 {
        let idx = indices[i];
        if idx >= ranks[i].len() {
            temp.push(4);
        } else {
            temp.push(ranks[i][idx]);
        }
    }

    let mut temp2 = temp.clone();
    temp2.sort();
    temp2.dedup();
    if !(temp2 == [1, 2, 3, 4] || temp2 == [1, 2, 3]) {
        return false;
    }

    if temp2 == [1,2,3,4] {
        let mut n_indices = indices.clone();
        for i in 0..4 {
            n_indices[i] += 1;
        }
        let ret = rec(ranks, n_indices, idx+1, n, ans);
        if ret {
            for i in 0..4 {
                if temp[i] == 4 {
                    ans.push(i+1);
                }
            }
        }
        return ret;
    }

    let dup = get_dup(&temp);
    for i in 0..4 {
        if temp[i] == dup {
            let mut n_indices = indices.clone();
            for j in 0..4 {
                if i != j {
                    n_indices[j] += 1;
                }
            }

            if rec(ranks, n_indices, idx+1, n, ans) {
                ans.push(i+1);
                return true
            }

        }
    }

    return false
}

fn main() {
    input!{
      n: usize,
      a: usize,
      b: usize,
      c: usize,
      d: usize,
      mut aa: [usize; a],
      mut bb: [usize; b],
      mut cc: [usize; c],
      mut dd: [usize; d],
    }
    if n*3 != (a+b+c+d) {
        return println!("{}", "No")
    }

    let ranks = [aa, bb, cc, dd];
    let indices = [0; 4];
    let mut ans = vec![];
    let ret = rec(&ranks, indices, 0, n, &mut ans);
    println!("{}", if ret { "Yes" } else { "No" });
    if ret {
        ans.reverse();
        for a in ans {
            println!("{}", a);
        }
    }
}
