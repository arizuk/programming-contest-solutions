fn bs(size: usize, f: fn(usize) -> i32) -> i32  {
  let mut s = 0;
  let mut e = size - 1;

  while s <= e {
    let i = (s + e) / 2;
    // println!("s={} e={} i={}", s, e, i);
    match f(i) {
      -1 => s = i + 1,
      0 => return i as i32,
      1 => e = i - 1,
      _ => panic!('_')
    }
  }
  -1
}

fn main() {
  let f = |i| {
    if i < 5 { -1 } else { 1 }
  };
  println!("{}", bs(20, f));
}