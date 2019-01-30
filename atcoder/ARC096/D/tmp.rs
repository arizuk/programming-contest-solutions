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
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
struct Node {
    v: i64,
    i: usize,
}

#[derive(Debug)]
struct Heap {
    last: usize,
    buf: Vec<Option<Node>>,
}

impl Heap {
    fn new (size: usize) -> Heap {
        let mut s = 1;
        while s < size {
            s *= 2;
        }
        return Heap { buf: vec![None; s], last: 0 }
    }

    fn add(&mut self, v: Node) {
        let mut cur = self.last;

        self.buf[self.last] = Some(v);
        self.last += 1;

        loop {
            if cur == 0 { break; }

            let par = (cur-1)/2;
            let par_v = self.buf[par].unwrap();
            if par_v > v {
                break;
            }

            let tmp = self.buf[par];
            self.buf[par] = Some(v);
            self.buf[cur] = tmp;
            cur = par;
        }
    }

    // fn pop(&mut self) -> Option<Node> {
    //     if self.last == 0 {
    //         return None;
    //     }

    //     let peek = self.buf[0].unwrap();

    //     let v = self.buf[self.last-1];
    //     self.buf[self.last-1] = None;
    //     self.last -= 1;

    //     if self.last == 0 {
    //         return Some(peek);
    //     }

    //     let mut cur = 0;
    //     self.buf[cur] = v;
    //     loop {
    //         let left = (cur+1) * 2 - 1;
    //         let right = (cur+1) * 2;

    //         if left >= self.buf.len() {
    //             break;
    //         }

    //         let cur_v = self.buf[cur].unwrap().v;
    //         let left_v = match self.buf[left] {
    //             Some(n) => n.v,
    //             None => 0,
    //         };
    //         let right_v = match self.buf[right] {
    //             Some(n) => n.v,
    //             None => 0,
    //         };

    //         if left_v <= cur_v && right_v <= cur_v {
    //             break;
    //         }

    //         if left_v < right_v {
    //             let tmp = self.buf[right];
    //             self.buf[right] = self.buf[cur];
    //             self.buf[cur] = tmp;
    //             cur = right;
    //         } else {
    //             let tmp = self.buf[left];
    //             self.buf[left] = self.buf[cur];
    //             self.buf[cur] = tmp;
    //             cur = left;
    //         }
    //     }

    //     Some(peek)
    // }
}

fn main() {
    let mut heap = Heap::new(3);
    heap.add(Node { v: 1, i: 1 });
    heap.add(Node { v: 5, i: 1 });
    heap.add(Node { v: 2, i: 1 });
    debug!(heap);
    // debug!(heap.pop());
    // debug!(heap.pop());
    // debug!(heap.pop());
    // debug!(heap.pop());
}
