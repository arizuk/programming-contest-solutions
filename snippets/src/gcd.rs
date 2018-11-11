#[allow(dead_code)]
#[snippet = "gcd"]
fn gcd(a: u64, b:u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[allow(dead_code)]
#[snippet = "lcm"]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[test]
fn test_gcd() {
  assert_eq!(gcd(10, 15), 5);
}

#[test]
fn test_lcm() {
  assert_eq!(lcm(10, 15), 30);
}