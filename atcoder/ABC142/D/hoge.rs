pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

use std::cmp::max;
fn main() {
    for i in 10..40 {
        for j in i..40 {
            let mut ans = vec![];
            for k in 1..max(i, j)+1 {
                if i%k ==0 && j%k==0 {
                    ans.push(k);
                }
            }
            println!("i={} j={} ans={:?} gcd={}", i, j, ans, gcd(i as _, j as _));
        }
    }
}