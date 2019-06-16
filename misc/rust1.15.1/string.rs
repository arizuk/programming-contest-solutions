fn main() {
    let a = ['a', 'b', 'c'];
    let st: String = a.iter().cloned().collect();
    println!("st={:?}", st);

    let st: String = "abc".to_string();
    println!("st={:?}", st);

    let st: Vec<char> = "abc".chars().collect();
    println!("st={:?}", st);

    let st: String = "abc".to_string();
    let b: Vec<char> = st.chars().collect();
    println!("b={:?}", b);

    for c in st.chars() {
        println!("c={:?}", c);
    }

    let stn = "12345";
    let n = stn.parse::<usize>().unwrap();
    println!("n+1={:?}", n+1);

    println!("to_digit");
    println!("9={:?}", '9'.to_digit(10));
    println!("c={:?}", 'c'.to_digit(16));

    // 2進数に変換
    let a = format!("{:b}", 15);
    println!("a={}", a);

    // 2進数をparse
    let b = usize::from_str_radix(&a, 2).unwrap();
    println!("b={:?}", b);
}