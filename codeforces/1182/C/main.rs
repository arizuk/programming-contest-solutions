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

fn vowel_index(c: char) -> usize {
    match c {
        'a' => 0,
        'i' => 1,
        'u' => 2,
        'e' => 3,
        'o' => 4,
        _ => 99,
    }
}

fn main() {
    input!{
      n: usize,
      orig_words: [chars; n],
    }
    let mut vowel_nums = vec![];
    let mut words = vec![];
    for w in orig_words.iter() {
        let mut num = 0;
        let mut v_index = 0;
        for i in 0..w.len() {
            let temp = vowel_index(w[i]);
            if temp <= 4 {
                num += 1;
                v_index = temp;
            }
        }
        let st = w.into_iter().collect::<String>();
        words.push((num, v_index, st));
        vowel_nums.push(num);
    }
    vowel_nums.sort();
    vowel_nums.dedup();

    let m = vowel_nums.len();
    let mut table = vec![vec![vec![]; 5]; m];

    for w in words.into_iter() {
        let idx = vowel_nums.binary_search(&w.0).unwrap();
        let v_index = w.1;
        table[idx][v_index].push(w.2);
    }

    use std::collections::VecDeque;
    let mut pair1 = VecDeque::new();
    let mut pair2 = VecDeque::new();

    for i in 0..table.len() {
        // 母音の数が同じものの組み合わせ
        let mut rems = vec![];
        for vi in 0..5 {
            // 最後の母音まで一緒

            let mut cur = vec![];
            for word in table[i][vi].iter() {
                cur.push(word);
                if cur.len() == 2 {
                    pair2.push_back((cur[0], cur[1]));
                    cur = vec![];
                }
            }
            for word in cur.into_iter() {
                rems.push(word)
            }
        }

        for w in rems.chunks(2) {
            if w.len() == 2 {
                pair1.push_back((w[0], w[1]));
            }
        }
    }
    // debug!(pair1);
    // debug!(pair2);
    let mut ans = min(pair1.len(), pair2.len());
    if pair2.len() >= pair1.len() {
        ans = (pair1.len() + pair2.len()) / 2;
    }
    println!("{}", ans);
    for _ in 0..ans {
        let mut p1 = pair1.pop_front();
        if p1.is_none() {
            p1 = pair2.pop_front();
        }
        let p2 = pair2.pop_front();
        match (p1, p2) {
            (Some((a, b)), Some((c, d))) => {
                println!("{} {}", a, c);
                println!("{} {}", b, d);
            },
            _ => (),
        }
    }
}
