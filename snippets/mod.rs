const MAX : usize = 10;
const MOD : usize = 1e9 as usize + 7;

fn init_fact() -> (Vec<usize>, Vec<usize>, Vec<usize>) {
  let mut fac = vec![0; MAX];
  let mut inv = vec![0; MAX];
  let mut finv = vec![0; MAX];
  fac[0] = 1;
  fac[1] = 1;
  finv[0] = 1;
  finv[1] = 1;
  inv[1] = 1;
  for i in 2..MAX {
    fac[i] = fac[i-1] * i % MOD;
    inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
    finv[i] = finv[i-1] * inv[i] % MOD;
  }
  return (fac, inv, finv)
}

fn main() {
  println!("{}", MOD);
  let (fac, inv, finv) = init_fact();
  println!("fac={:?}", fac);
  println!("inv={:?}", inv);
  println!("finv={:?}", finv);
}