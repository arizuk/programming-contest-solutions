fn main() {
    let aa = vec![1, 5, 2, 3, 4];
    // let aa = vec![1, 4, 3, 2, 5];
    let n = aa.len();
    let mut ans = vec![];
    for i in 0..n {
        let mut s = 0;
        for j in 0..i {
            if aa[j] < aa[i] {
                s += aa[j]
            }
        }
        ans.push(s);
    }
    println!("ans={:?}", ans);
}
