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

fn main() {
    input!{
        n: i32,
        k: i32,
    }

    let mut ans = 0;
    for a in 1..(n + 1) {
        for i in (a / k)..(n + 1) {
            let bc = i * k - a;
            if bc > n {
                break;
            };
            if bc <= 0 {
                continue;
            };

            for b in 1..(bc - 1) {
                let c = bc - b;
                if (b + a) % k == 0 && (c + a) % k == 0 {
                    ans += 1;
                }
            }

            // for j in (b/k)..(n+1) {
            //     let c = j * k - b;
            //     if c > n { break };
            //     if c <= 0 { continue };

            //     if (a + c) % k == 0 {
            //         // println!("{} {} {}", a, b, c);
            //         ans += 1;
            //     }
            // }
        }
    }
    println!("{}", ans);
}
