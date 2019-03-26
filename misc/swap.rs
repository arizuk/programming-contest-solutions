use std::collections::HashMap;

fn main() {
  let mut m1: HashMap<u64, u64> = HashMap::new();
  let mut m2: HashMap<u64, u64> = HashMap::new();
  m1.insert(1, 2);
  m1.insert(2, 3);
  m2.insert(3, 4);
  m2.insert(4, 5);
  println!("m1={:?} m2={:?}", m1, m2);
  std::mem::swap(&mut m1, &mut m2);
  println!("m1={:?} m2={:?}", m1, m2);
}