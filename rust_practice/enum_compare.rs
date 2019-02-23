#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Hoge {
  A,
  B
}

fn main() {
  let a = Hoge::A;
  let b = Hoge::B;
  let a2 = Hoge::A;

  println!("{:?}", a > b);
  println!("{:?}", a == b);
  println!("{:?}", a < b);
  println!("{:?}", a == a2);
}