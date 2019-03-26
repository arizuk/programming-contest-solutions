fn main() {
  let v = ["hoge", "fuga"];
  for i in 0..v.len() {
    let s = v[i].to_string();
    println!("{}", s);
    println!("is_hoge={}", s == "hoge");

    let s = &v[i];
    println!("{}", s);
    println!("is_hoge={}", *s == "hoge");
  }
}