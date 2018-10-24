fn binary_search(t: usize, ks: &[usize]) -> bool {
    println!("##--- search {} in {:?}", t, ks);
    let (mut s, mut e) = (0, ks.len() - 1);
    while s <= e {
        let i = (s + e) / 2;
        println!("s={} e={} i={} k={}", s, e, i, ks[i]);
        match ks[i] {
            x if x == t => return true,
            x if x < t => s = i + 1,
            _ => e = i - 1,
        }
    }
    false
}

fn solve(m: usize, ks: &[usize]) -> String {
    let n = ks.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if m < ks[i] + ks[j] + ks[k] {
                    continue;
                }
                let t = m - ks[i] - ks[j] - ks[k];
                if binary_search(t, ks) {
                    return "Yes".to_string();
                }
            }
        }
    }
    "No".to_string()
}

fn main() {
    println!("{}", binary_search(1, &[1]));
    println!("{}", binary_search(2, &[1]));
    println!("{}", binary_search(1, &[1, 5]));
    println!("{}", binary_search(5, &[1, 5]));
    println!("{}", binary_search(7, &[1, 5]));
    println!("{}", binary_search(10, &[1, 3, 5, 11, 18, 20]));
    println!("{}", binary_search(11, &[1, 3, 5, 11, 18, 20]));
}
