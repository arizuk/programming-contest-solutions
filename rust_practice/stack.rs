#[derive(Debug)]
struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
  fn new(data: T) -> Self {
    Node { data, next: None }
  }
}

#[derive(Debug)]
struct Stack<T> {
  top: Option<Box<Node<T>>>,
  size: usize,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Stack { top: None, size: 0 }
  }

  fn push(&mut self, data: T) {
    self.size += 1;
    self.top = Some(Box::new(Node {
      data: data,
      next: self.top.take(),
    }))
  }

  fn pop(&mut self) -> Option<T> {
    match self.top.take() {
      Some(mut node) => {
        self.size -= 1;
        self.top = node.next.take();
        Some(node.data)
      },
      None => None,
    }
  }
}

fn main() {
  let mut stack = Stack::new();
  stack.push(1);
  stack.push(3);
  stack.push(5);
  println!("stack={:?}", stack);

  for i in 0..5 {
    println!("{:?}", stack.pop());
  }
}