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

pub mod ds {
    use std::cmp::Ordering::*;
    type Link<T> = Option<Box<Node<T>>>;
    #[derive(Debug)]
    struct Node<T> {
        left: Link<T>,
        right: Link<T>,
        key: T,
        priority: u32,
        count: usize,
    }
    impl<T> Node<T> {
        fn new(key: T, priority: u32) -> Link<T> {
            Some(Box::new(Node {
                left: None,
                right: None,
                key: key,
                priority: priority,
                count: 1,
            }))
        }
        fn update(&mut self) {
            self.count = 1 + count(&self.left) + count(&self.right);
        }
    }
    #[doc = " `Treap` is a binary search tree data structure that maintain a dynamic set"]
    #[doc = " of ordered keys and allow binary searches among the keys,"]
    #[doc = " [https://en.wikipedia.org/wiki/Treap](https://en.wikipedia.org/wiki/Treap)"]
    #[derive(Debug)]
    pub struct Treap<T> {
        random_state: XorShift,
        root: Link<T>,
    }
    impl<T: PartialOrd> Treap<T>
    where
        T: Clone,
    {
        pub fn new(seed: u32) -> Self {
            Treap {
                random_state: XorShift { state: seed },
                root: None,
            }
        }
        pub fn clear(&mut self) {
            self.root = None
        }
        pub fn is_empty(&self) -> bool {
            count(&self.root) == 0
        }
        pub fn insert(&mut self, key: T) {
            if !self.contains(&key) {
                self.root = insert(self.root.take(), key, &mut self.random_state);
            }
        }
        pub fn contains(&self, key: &T) -> bool {
            find(&self.root, key).is_some()
        }
        pub fn erase(&mut self, key: &T) {
            self.root = erase(self.root.take(), key);
        }
        pub fn nth(&self, n: usize) -> &T {
            match rank(&self.root, n) {
                Some(r) => &r.key,
                None => panic!(),
            }
        }
    }
    fn find<T: PartialOrd>(node: &Link<T>, key: &T) -> Option<usize> {
        match node {
            None => None,
            Some(node) => match node.key.partial_cmp(key).unwrap() {
                Equal => Some(count(&node.left)),
                Greater => find(&node.left, key),
                Less => match find(&node.right, key) {
                    None => None,
                    Some(pos) => Some(count(&node.left) + 1 + pos),
                },
            },
        }
    }
    fn count<T>(node: &Link<T>) -> usize {
        match node {
            None => 0,
            Some(node) => node.count,
        }
    }
    fn rotate_left<T>(node: Link<T>) -> Link<T> {
        let mut node = node.unwrap();
        let mut r = node.right.take().unwrap();
        node.right = r.left.take();
        node.update();
        r.left = Some(node);
        Some(r)
    }
    fn rotate_right<T>(node: Link<T>) -> Link<T> {
        let mut node = node.unwrap();
        let mut l = node.left.take().unwrap();
        node.left = l.right.take();
        node.update();
        l.right = Some(node);
        Some(l)
    }
    fn insert<T: PartialOrd>(node: Link<T>, key: T, rand: &mut XorShift) -> Link<T> {
        match node {
            None => Node::new(key, rand.next()),
            Some(mut node) => {
                match node.key.partial_cmp(&key).unwrap() {
                    Less => {
                        node.right = insert(node.right.take(), key, rand);
                        if priority(&node.right) < node.priority {
                            node = rotate_left(Some(node)).unwrap();
                        }
                    }
                    _ => {
                        node.left = insert(node.left.take(), key, rand);
                        if priority(&node.left) < node.priority {
                            node = rotate_right(Some(node)).unwrap();
                        }
                    }
                }
                node.update();
                Some(node)
            }
        }
    }
    fn priority<T>(node: &Link<T>) -> u32 {
        match *node {
            None => panic!(),
            Some(ref node) => node.priority,
        }
    }
    fn min<T>(node: &Link<T>) -> &Link<T> {
        match node {
            Some(x) => match x.left {
                Some(_) => min(&x.left),
                None => node,
            },
            None => panic!(),
        }
    }
    fn erase<T: PartialOrd>(node: Link<T>, key: &T) -> Link<T>
    where
        T: Clone,
    {
        match node {
            None => panic!(),
            Some(mut node) => match node.key.partial_cmp(key).unwrap() {
                Equal => {
                    if node.left.is_none() {
                        node.right
                    } else if node.right.is_none() {
                        node.left
                    } else {
                        node.key = match min(&node.right) {
                            Some(m) => m.key.clone(),
                            None => panic!(),
                        };
                        node.right = erase(node.right.take(), &node.key);
                        node.update();
                        Some(node)
                    }
                }
                Less => {
                    node.right = erase(node.right.take(), key);
                    node.update();
                    Some(node)
                }
                Greater => {
                    node.left = erase(node.left.take(), key);
                    node.update();
                    Some(node)
                }
            },
        }
    }
    fn rank<T>(node: &Link<T>, r: usize) -> &Link<T> {
        match node {
            Some(c) => {
                let left = count(&c.left);
                if left == r {
                    node
                } else if left < r {
                    rank(&c.right, r - left - 1)
                } else {
                    rank(&c.left, r)
                }
            }
            None => panic!(),
        }
    }
    #[derive(Debug)]
    struct XorShift {
        state: u32,
    }
    impl XorShift {
        fn next(&mut self) -> u32 {
            self.state = xor_shift(self.state);
            self.state
        }
    }
    fn xor_shift(state: u32) -> u32 {
        let mut x = state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }
}

use ds::Treap;

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      aa: [usize; n],
    }

    let mut pos = vec![0; n+1];
    for i in 0..n {
        let a = aa[i];
        pos[a] = i;
    }

    let mut treap = Treap::new(10);
}
