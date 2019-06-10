#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;

const MAX: u64 = 2750131;

pub fn gen_prime_table(n: u64) -> Vec<bool> {
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n as usize {
        if is_prime[i] {
            let mut j = 2;
            while i * j <= n as usize {
                is_prime[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    is_prime
}

fn get_g_divisor(n: usize) -> usize {
    let mut i = 2;
    while i*i <= n {
        if n%i == 0 {
            return n/i;
        }
        i += 1;
    }
    1
}

fn solve() {
    input!{
      n: usize,
      mut aa: [usize; 2*n],
    }
    let is_primes = gen_prime_table(MAX);
    let mut all_primes = vec![];
    for v in 2..MAX+1 {
        if is_primes[v as usize] {
            all_primes.push(v);
        }
    }

    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut primes = vec![];
    let mut not_primes = vec![];
    for &a in aa.iter() {
        *map.entry(a).or_insert(0) += 1;
        if is_primes[a] {
            primes.push(a);
        } else {
            not_primes.push(a);
        }
    }
    // debug!(map);
    primes.sort();
    not_primes.sort();
    let mut ans = vec![];
    for &v in not_primes.iter().rev() {
        let mut cnt = *map.get(&v).unwrap();
        if cnt >= 1 {
            ans.push(v);
            let gv = get_g_divisor(v);
            *map.entry(v).or_insert(0) -= 1;
            *map.entry(gv).or_insert(0) -= 1;
            // debug!(v, gv, map);
        }
    }

    for &v in primes.iter() {
        let mut cnt = *map.get(&v).unwrap();
        if cnt >= 1 {
            ans.push(v);
            *map.entry(v).or_insert(0) -= 1;
            let np = all_primes[v-1];
            *map.entry(np as usize).or_insert(0) -= 1;
            // debug!(v, np, map);
        }
    }
    for a in ans {
        print!("{} ", a);
    }
    println!();
}


fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
