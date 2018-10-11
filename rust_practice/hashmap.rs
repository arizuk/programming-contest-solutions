use std::collections::HashMap;

fn main() {
  let mut map: HashMap<String, i32> = HashMap::new();
  let s = vec!["hoge", "fuga", "hoge"];
  for i in 0..s.len() {
    let e = map.entry(s[i].to_string()).or_insert(0);
    *e += 1;
  }
  println!("map={:?}", map);
}