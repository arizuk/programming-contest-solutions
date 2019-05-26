const MOD: usize = 13;
pub fn mod_pow(b: usize, p: usize) -> usize {
    if p == 1 {
        return b;
    }
    let mut ret = mod_pow(b * b % MOD, p / 2) % MOD;
    if p % 2 == 1 {
        ret = ret * b % MOD;
    }
    ret
}

fn main() {
  for i in 1..20 {
    // 逆元計算
    println!("i={} inv={} (mod {})", i, mod_pow(i, MOD-2), MOD);
  }
}