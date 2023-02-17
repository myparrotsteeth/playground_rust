use std::cmp::Ordering;

//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Box<TreeNode>>,
  pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }

  pub fn push(&mut self, val: i32) {
    match self.val.cmp(&val) {
      Ordering::Greater => {
        if let Some(n) = &mut self.left {
          (*n).push(val);
        } 
        else {
          self.left = Some(Box::new(TreeNode::new(val)));
        }
      },
      Ordering::Equal => (),
      Ordering::Less => {
        if let Some(n) = &mut self.right {
          (*n).push(val);
        } 
        else {
          self.right = Some(Box::new(TreeNode::new(val)));
        }
      },
    } 
  }

  pub fn print(&self) {
    println!("{}", self.val);
    if let Some(n) = &self.left {
      (*n).print();
    }
    if let Some(n) = &self.right {
      (*n).print();
    }    
  }

  pub fn right_side_print(&self) {
    println!("{}", self.val);
    if let Some(n) = &self.right {
      (*n).right_side_print();
    }
    else {
      println!("Done.");
    }
  }
}


fn main() {
  let mut tree = TreeNode::new(3);
  tree.right_side_print();
  tree.push(5);
  tree.right_side_print();
  tree.push(6);
  tree.push(4);
  tree.right_side_print();
}