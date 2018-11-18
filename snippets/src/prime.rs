#[allow(dead_code)]
#[snippet = "gen_prime_table"]
fn gen_prime_table(n: u64) -> Vec<bool> {
  let mut is_prime = vec![true; n as usize + 1];
  is_prime[0] = false;
  is_prime[1] = false;
  let mut i = 2;
  while i*i <= n as usize {
    if is_prime[i] {
      let mut j = 2;
      while i*j <= n as usize {
        is_prime[i*j] = false;
        j += 1;
      }
    }
    i += 1;
  }
  is_prime
}

#[allow(dead_code)]
#[snippet = "gen_primes"]
fn gen_primes(n: u64) -> Vec<u64> {
  let is_prime = gen_prime_table(n);
  let mut primes = vec![];
  for i in 2..n+1 {
    if is_prime[i as usize] {
      primes.push(i);
    }
  }
  primes
}

#[test]
fn test_gen_prime_table() {
  assert_eq!(gen_prime_table(7), vec![false, false, true, true, false, true, false, true]);
}

#[test]
fn test_gen_primes() {
  assert_eq!(gen_primes(7), vec![2, 3, 5, 7]);
  assert_eq!(gen_primes(444777).len(), 37326);
}