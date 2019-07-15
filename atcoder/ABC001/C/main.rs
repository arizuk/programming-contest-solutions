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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      mut deg: usize,
      dis: f64,
    }
    deg *= 10;

    let mut table = [
        (01125, 03375, "NNE"),
        (03375, 05625, "NE"),
        (05625, 07875, "ENE"),
        (07875, 10125, "E"),
        (10125, 12375, "ESE"),
        (12375, 14625, "SE"),
        (14625, 16875, "SSE"),
        (16875, 19125, "S"),
        (19125, 21375, "SSW"),
        (21375, 23625, "SW"),
        (23625, 25875, "WSW"),
        (25875, 28125, "W"),
        (28125, 30375, "WNW"),
        (30375, 32625, "NW"),
        (32625, 34875, "NNW"),
    ];


    let mut table2 = [
        0.25,
        1.55,
        3.35,
        5.45,
        7.95,
        10.75,
        13.85,
        17.15,
        20.75,
        24.45,
        28.45,
        32.65,
    ];
    let table2: Vec<f64> = table2.into_iter().map(|v| v * 60.0).collect();

    let mut dir = "N";
    for &(l, r, d) in table.iter() {
        if deg >= l && deg < r {
            dir = d;
            break;
        }
    }

    let mut w = 0;
    for i in 0..table2.len() {
        let uppper = table2[i];
        if dis < uppper {
            w = i;
            break;
        }
    }
    if dis >= table2[table2.len()-1] {
        w = 12;
    }

    if w == 0 {
        puts!("C 0");
    } else {
        puts!("{} {}", dir, w);
    }
}
