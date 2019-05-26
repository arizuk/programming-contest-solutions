fn random() -> usize {
  use std::fs::File;
  use std::io::Read;
  let mut f = File::open("/dev/urandom").unwrap();
  let mut buf = [0u8; 1];
  f.read_exact(&mut buf).unwrap();
  buf[0] as _
}

fn main() {
  use std::collections::HashMap;
  let mut map = HashMap::new();
  for i in 1..10 {
    *map.entry(i).or_insert(0) += random();
  }

  {
    let record = map.get(&1);
    println!("record={:?}", record);
  }

  let mut a: Vec<(usize,usize)> = map.into_iter().collect();
  a.sort_by_key(|v| v.1);
  for (k, v) in a {
    println!("k={:?} v={:?}", k, v);
  }
}