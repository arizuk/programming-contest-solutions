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
use std::io::{stdout, stdin, BufWriter, Write};

struct RollingHash {
    modulo: usize,
    power: Vec<usize>,
    hash: Vec<usize>,
}

impl RollingHash {
    fn new(s: &[u8], base: usize, modulo: usize) -> Self {
        let n = s.len();

        let mut power = vec![1; n+1];
        let mut hash = vec![0; n+1];
        for i in 0..n {
            power[i+1] = power[i] * base % modulo;
            hash[i+1] = (hash[i] * base + s[i] as usize) % modulo;
        }
        RollingHash {
            modulo: modulo,
            power: power,
            hash: hash
        }
    }

    fn get(&self, l: usize, r: usize) -> usize {
        (self.hash[r] + self.modulo - (self.hash[l] * self.power[r-l]) % self.modulo) % self.modulo
    }
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      s: String,
    }

    let rh1 = RollingHash::new(s.as_bytes(), 1009, 1000000007);
    let rh2 = RollingHash::new(s.as_bytes(), 1007, 1000000009);

    let is_ok = |len| {
        if len == 0 {
            return true;
        }
        if len > n/2 {
            return false;
        }


        let mut memo1 = vec![0; n];
        let mut memo2 = vec![0; n];
        for i in 0..n-len+1 {
            memo1[i] = rh1.get(i, i+len);
            memo2[i] = rh2.get(i, i+len);
        }

        for i in 0..n-len+1 {
            let ih1 = memo1[i];
            let ih2 = memo2[i];
            for j in i+len..n-len+1 {
                if memo1[j] == ih1 && ih2 == memo2[j] {
                    return true;
                }
            }
        }
        false
    };

    let mut ng = n as isize;
    let mut ok = 0;
    while ng-ok > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid as usize) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    puts!("{}\n", ok);
}
