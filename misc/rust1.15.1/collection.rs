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

    for i in 0..10 {
        if a.iter().any(|&v| v== i) {
            println!("a={:?} Contains {}", a, i);
        }
    }
    if !a.iter().all(|v| *v % 2 == 1 ) {
        println!("all a={:?} is not odd", a);
    }
    let b = [1, 3, 5];
    if b.iter().all(|v| *v % 2 == 1 ) {
        println!("all b={:?} is odd", b);
    }

    let c: Vec<usize> = vec![];
    if c.iter().all(|v| *v % 2 == 1 ) {
        println!("all returns true if the collection length is zero");
    }
}