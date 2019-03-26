fn main() {
  let a = 15;
  println!("{:#06b}", a);

  for i in 0..3 {
    let mask = (1 << i);
    println!("mask={} {:#06b}", i, a & !mask);
  }

  for i in 0..3 {
    let mask = (1 << i) - 1;
    println!("mask={} {:#06b}", i, a & mask);
  }

  for i in 0..3 {
    let mask = -1 << i;
    println!("mask={} {:#06b}", i, a & mask);
  }
}