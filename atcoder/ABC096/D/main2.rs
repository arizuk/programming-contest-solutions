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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};
use std::collections::HashSet;
use std::iter::FromIterator;

fn enum_primes(n: u64) -> Vec<u64>{
    let mut primes = vec![];
    let mut is_prime = vec![true; n as usize + 1];

    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n as usize {
        if is_prime[i] == true {
            primes.push(i as u64);
            let mut j = 2;
            while (i*j) as u64 <= n {
                is_prime[i*j] = false;
                j += 1;
            }
        }
    }
    return primes;
}

fn main() {
    input!{
      n: usize,
    }
    let primes = enum_primes(55556*5);
    let primes_set: HashSet<u64> = HashSet::from_iter(primes.clone());

    let mut prime_idx = 6;
    let mut ans = vec![2, 3, 5, 7, 13];
    let mut combi = [0; 4];

    for i in 6..(n+1) as u64 {
        let bit = 1 << i-1;

        loop {
            if prime_idx >= primes.len() {
                println!("Couldn't find answer {}", n);
                return;
            }
            let prime = primes[prime_idx];
            let mut ok = true;
            for j in 1u64..bit {
                if j.count_ones() != 4 { continue; }
                let mut idx = 0;
                for k in 0..i {
                    if j & (1 << k) > 0 {
                        combi[idx] = ans[k as usize];
                        idx += 1;
                    }
                }

                let mut sum = prime;
                for v in &combi {
                    sum += *v;
                }
                if primes_set.contains(&sum) {
                    ok = false;
                    break;
                }
            }

            prime_idx += 1;
            if ok {
                ans.push(prime);
                break;
            }
        }
    }
    for i in 0..ans.len() {
        if i == ans.len() - 1 {
            print!("{}", ans[i]);
        } else {
            print!("{} ", ans[i]);
        }
    }
    println!();
}
