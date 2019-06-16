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

  // zip && collect
  let a = vec!['a', 'b', 'c'];
  let b = vec![3, 2, 1];
  let map: HashMap<_, _> = a.iter().zip(b.iter()).collect();
  println!("map={:?}", map);


  let a = vec!['a', 'b', 'c'];
  let map: HashMap<_, _> = a.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
  println!("map={:?}", map);

  let keys: Vec<_> = map.keys().cloned().collect();
  println!("keys={:?}", keys);

  let values: Vec<_> = map.values().cloned().collect();
  println!("values={:?}", values);

  // 2-dimensional HashMap
  let mut map = HashMap::new();
  for i in 0..3 {
    for j in 1..4 {
      map.entry(i).or_insert_with(HashMap::<usize,usize>::new).insert(j, i*j);
    }
  }
  println!("map={:?}", map);

  // iterate
  for (k, v) in map.iter() {
    for (k2, v) in v.iter() {
      println!("k={:?} k2={}, v={:?}", k, k2, v);
    }
  }

  // Use tuple as key
  let mut map = HashMap::new();
  for i in 0..3 {
    for j in 1..4 {
      map.entry((i,j)).or_insert(i*j);
    }
  }
  println!("map={:?}", map);

  let string = "final fantasy".to_string();
  let mut map: HashMap<char, usize> = string.chars().map(|c| (c, 0)).collect();
  for c in string.chars() {
    // *map.entry(c).or_insert(0) += 1;
    *map.get_mut(&c).unwrap() += 1;
  }
  println!("map={:?}", map);
}