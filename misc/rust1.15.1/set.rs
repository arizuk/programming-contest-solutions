use std::collections::{HashSet, BTreeSet};

fn main() {
    let a = vec![8, 9, 2, 24, 1];
    let set: BTreeSet<_> = a.iter().cloned().collect();
    println!("# BtreeSet iter");
    println!("set={:?}", set);
    for v in set.iter() {
        println!("v={:?}", v);
    }
    let mut iter = set.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.last());

    let set: HashSet<_> = a.iter().cloned().collect();
    println!("# HashSet iter");
    println!("set={:?}", set);
    for k in set.iter() {
        println!("k={:?}", k);
    }
    for i in 0..10 {
        if set.contains(&i) {
            println!("Set contains {}", i);
        }
    }
}