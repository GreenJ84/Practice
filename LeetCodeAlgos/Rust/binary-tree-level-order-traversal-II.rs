// Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).

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

use std::rc::Rc;
use std::cell::RefCell;
struct Solution{}
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let None = root { return vec![]; }
        let levels: Vec<Vec<i32>> = Vec::new();
        let mut level_values: Vec<i32> = Vec::new();

        let mut level: Vec<Rc<RefCell<TreeNode>>> = vec![root];
        let mut next: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        while !level.is_empty() {
            for node in level {
                let node_borrow = node.borrow();
                level_values.push(node_borrow.val);
                if let Some(left) = node_borrow.left { next.push(left.clone()); }
                if let Some(right) = node_borrow.right { next.push(right.clone()); }
            }
            levels.push(level_values);
            level_values = Vec::new();
            level = next;
            next = Vec::new();
        }
        levels.iter().rev().cloned().collect()
    }
}


