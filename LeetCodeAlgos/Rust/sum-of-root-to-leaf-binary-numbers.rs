// You are given the root of a binary tree where each node has a value 0 or 1. Each root-to-leaf path represents a binary number starting with the most significant bit.
  // For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.

// For all leaves in the tree, consider the numbers represented by the path from the root to that leaf. Return the sum of these numbers.

// The test cases are generated so that the answer fits in a 32-bits integer.

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
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
}

struct Solution;


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum = 0;
    let mut path_string = String::new();

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, value: &mut String, curr_sum: &mut i32) {
      if node.is_none() { return; }

      let node = node.unwrap();
      let ch: char = char::from_digit(node.borrow().val as u32, 10).unwrap();
      value.push(ch);

      match (&node.borrow().left, &node.borrow().right) {
        // Leaf
        (None, None) => {
            println!("value: {}", value);
          let parsed_number = match i32::from_str_radix(value, 2) {
            Ok(num) => num,
            Err(e) => { panic!("Error parsing binary string: {}", e); }
          };
          *curr_sum += parsed_number;
        },
        // Not leaf
        (left, right) => {
          if left.is_some() {
            dfs(left.as_ref(), value, curr_sum);
          }
          if right.is_some() {
            dfs(right.as_ref(), value, curr_sum);
          }
        }
      }

      value.pop();
    }

    dfs(root.as_ref(), &mut path_string, &mut sum);
    sum
  }
}