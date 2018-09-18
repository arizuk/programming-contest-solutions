fn add(x: i32, y: i32) -> i32 {
  x + y
}

fn main() {
  let f = add;
  println!("{}", f(1, 3));
}
