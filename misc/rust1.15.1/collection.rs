fn main() {
    let a = vec![1, 2, 3, 4];
    println!("a={:?}", a);
    let sum = a.iter().sum::<usize>();
    assert_eq!(sum, 10usize);
    println!("sum={:?}", sum);

    // cast
    let sum = a.iter().sum::<usize>() as u64;
    assert_eq!(sum, 10u64);
    println!("sum={:?}", sum);

    let product = a.iter().product::<usize>();
    assert_eq!(product, 24usize);
    println!("product={:?}", product);

    let chunks: Vec<&[usize]> = a.chunks(3).collect();
    for v in chunks.iter() {
        // v is &&[usize]
        println!("v={:?} sum={}", v, v.iter().sum::<usize>());
        // v[0] is usize
    }
}