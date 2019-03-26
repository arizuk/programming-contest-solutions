fn gcd(a: usize, b:usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
  assert_eq!(gcd(12, 8), 4);
  assert_eq!(lcm(5, 12), 60);
}