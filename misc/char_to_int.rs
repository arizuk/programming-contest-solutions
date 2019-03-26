fn main() {
  let s = ['a', 'b', 'c', 'd'];
  for &c in &s {
    println!("char={} i={}", c, c as u32 - 'a' as u32);
  }
}