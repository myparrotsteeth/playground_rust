

#[derive(Debug)]
struct Node {
  value: i32,
  next: Option<Box<Node>>
}

impl Node {

  fn create(val: i32) -> Option<Box<Node>> {
    Self::create_from_node(Node {value: val, next: None})
  }

  fn create_from_node(node: Node) -> Option<Box<Node>> {
    Some(Box::new(node))
  }

  fn append(&mut self, val: i32) {
    match &mut self.next {
      None => self.next = Node::create(val),
      Some(el) => el.append(val)
    }
    ;
  }

  fn insert_head(&mut self, val: i32) {
    match &mut self.next {
      None => {
        self.next = Node::create(self.value);
        self.value = val;
      },
      Some(el) => {
        let mut newnode = Node {
          value:val,
          next: None
         // next: Node::create_from_node(*self) 
        };                                                                                                                                                                                                                                                                                                                                                                                                                            ;
         // self = newnode;
      }
    }
  }

  fn insert_into(&mut self, val: i32) {

  }

}

fn main() {
  let mut first = Node {value: 1, next: None };
  first.append(3);
  first.append(4);
  dbg!(first);
}