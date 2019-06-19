fn main() {
    let n = 3;
    let aa = [1, 2, 3];
    let k = 2;

    let mut r = 0; // 条件を満たすindex
    let mut cur = aa[r];
    let mut ans = 0;
    let ok = |v| v >= k;

    // しゃくとり法
    for l in 0..n {
        if r < l {
            r = l;
            cur = aa[r];
        }
        while !ok(cur) && r < n {
            r += 1;
            if r < n {
                cur += aa[r];
            }
        }
        ans += n-r;
        cur -= aa[l];
    }
    println!("{}", ans);
}