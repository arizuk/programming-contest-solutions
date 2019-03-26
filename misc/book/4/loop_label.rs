fn main() {
  'outer: for i in 1..11 {
    for j in 1..11 {
      if i * j == 50 {
        break 'outer;
      } else {
        println!("{}", i * j);
      }
    }
  }
}