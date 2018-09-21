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


fn median(a: &[usize], x: usize) -> bool {
    let mut s: Vec<i32> = vec![0; a.len()];
    s[0] = if a[0] >= x { 1 } else { -1 };
    for i in 1..a.len() {
        s[i] = s[i-1] + if a[i] >= x { 1 } else { -1 };
    }
    println!("{:?} a={:?} x={}", s, a, x);
    // println!("x={}", x);

    let mut cnt = 0;
    let mut ttl = 0;
    for l in 0..a.len() {
        for r in l..a.len() {
            let sum = s[r] - if l >= 1 { s[l-1] } else { 0 };
            if sum >= 0 { cnt += 1 };
            // println!("{:?} {}", &a[l..(r+1)], sum);
            ttl += 1;
        }
    }
    // println!("x={} cnt={} ttl={}", x, cnt, ttl);

    let half = if ttl % 2 == 1 { ttl/2 + 1 } else { ttl/2 };
    if cnt >= half { true } else { false }
}

fn solve(a: &[usize], b: &[usize]) -> usize {
  let mut s = 0i32;
  let mut e = a.len() as i32 - 1;

  let mut last_x = 0;
  while s <= e {
    let i = (s + e) / 2;
    // println!("s={} e={} i={}", s, e, i);

    let x = b[i as usize];
    if median(a, x) {
        // println!("{} is ok", x);
        last_x = x;
        s = i + 1;
    } else {
        // println!("{} is ng", x);
        e = i - 1;
    }
  }
  last_x
}

fn main() {
    input!{
        n: usize,
        a: [usize; n]
    }
    let mut b = a.to_vec();
    b.sort();
    println!("{}", solve(&a, &b));
}
