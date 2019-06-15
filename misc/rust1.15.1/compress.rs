use std::collections::HashMap;
fn main() {
    let a = vec![0, 99, 1, 1234, 2, 1];
    let mut b: Vec<_> = a.iter().cloned().collect();
    b.sort();
    b.dedup();
    println!("b={:?}", b);
    let map: HashMap<_, _> = b.into_iter().zip(0..).collect();
    for a in a.iter() {
        println!("a={:?} idx={:?}", a, map[a]);
    }
}