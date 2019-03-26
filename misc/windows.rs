fn main() {
    let a = vec![1, 2, 3, 4, 5];
    for w in a.windows(3) {
        println!("{:?}", w);
    }
}
