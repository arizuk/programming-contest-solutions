use std::cmp;

fn solve(ns: &[i32]) -> i32 {
    let n = ns.len();
    let mut d = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                // println!("{}-{}-{}", i, j, k);
                let len = ns[i] + ns[j] + ns[k];
                let ma = cmp::max(ns[i], cmp::max(ns[j], ns[k]));
                let rest = len - ma;
                if rest > ma {
                    d = cmp::max(d, len);
                }
            }
        }
    }
    // println!("{}", d);
    d
}

fn main() {
    assert_eq!(12, solve(&[2, 3, 4, 5, 10]));
    assert_eq!(0, solve(&[4, 5, 10, 20]));
    println!("ok");
}
