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


fn set_depth(edges: &Vec<Vec<usize>>, cur:usize, d:usize, depth: &mut Vec<usize>) {
    depth[cur] = d;
    for &nd in edges[cur].iter() {
        set_depth(edges, nd, d+1, depth);
    }
}


fn dfs(edges: &Vec<Vec<usize>>, cur:usize, d:usize, used: &mut Vec<bool>, list: &mut Vec<(usize,usize)>) {
    if used[cur] {
        return
    }
    list.push((d, cur));
    for &nd in edges[cur].iter() {
        dfs(edges, nd, d+1, used, list);
    }
}

fn check(list: &Vec<(usize,usize)>, k: usize, m: usize) -> bool {
    if m == 0 {
        return k == 0
    }
    if k == 0 {
        return false
    }

    let sum = list.iter().map(|v| v.0).sum::<usize>();
    if k > sum {
        return false
    }
    let n = list.len();

    // 最大数
    let mut sum = 0;
    let mut max_m = 0;
    for i in 0..n {
        max_m += 1;
        sum += list[i].0;
        if sum >= k {
            if sum > k {
                max_m -= 1; // 入れ替え
            }
            break;
        }
    }

    // 最小数
    let mut sum = 0;
    let mut min_m = 0;
    for i in (0..n).rev() {
        min_m += 1;
        sum += list[i].0;
        if sum >= k {
            break;
        }
    }
    let ret = m >= min_m && m <= max_m;
    // if ret {
        // debug!(ret, list, k, m, min_m, max_m);
    // }
    return ret
}

fn main() {
    input!{
        n:usize,
        m:usize,
        k:usize,
        ps: [usize; n]
    }

    let mut edges = vec![vec![]; n];
    let mut root = 0;
    for i in 0..n {
        let p = ps[i];
        if p == 0 {
            root = i;
        } else {
            edges[p-1].push(i);
        }
    }
    let mut depth = vec![0; n];
    set_depth(&edges, root, 1, &mut depth);
    debug!(root, edges, depth);

    let mut cur_k = k;
    let mut cur_m = m;
    let mut used = vec![false; n];
    let mut ans = vec![];

    // debug!(n, m, k);
    for _ in 0..n {
        for i in 0..n {
            if depth[i] > cur_k {
                continue;
            }
            if used[i] {
                continue;
            }

            used[i] = true;
            let mut list = vec![];
            dfs(&edges, root, 1, &mut used, &mut list);
            list.sort();
            // debug!(ans, i, list, cur_k - depth[i], cur_m - 1);
            debug!(ans, i, list, depth[i], cur_k, cur_m);

            // これを使ってokなら
            if check(&list, cur_k - depth[i], cur_m - 1) {
                cur_k -= depth[i];
                cur_m -= 1;
                ans.push(i);
                if cur_m == 0 {
                    // debug!(ans);
                    for a in ans {
                        print!("{} ", a+1);
                    }
                    println!();
                    return;
                }
                break;
            } else {
                // debug!(i, "NG");
                used[i] = false;
            }
        }
    }
    println!("-1");
}
