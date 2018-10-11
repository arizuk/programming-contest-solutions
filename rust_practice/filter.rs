fn main() {
  let a = vec![1, 2, 3];
  let v = a.iter().filter(|&&v| v == 1).collect::<Vec<_>>();
  println!("{:?}", v);

  let v = a.iter().filter(|&&v| v == 1).count();
  println!("{}", v);

  for &v in a.iter().filter(|&&v| v >= 2) {
    println!("v={}", v);
  }
}