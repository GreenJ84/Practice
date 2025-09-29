// Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.

use std::rc::Rc;
use std::cell::RefCell;

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
impl Solution {
  pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() { return root; }

    let holder = Rc::new(RefCell::new(TreeNode::new(0)));

    {
      Self::dfs(root.unwrap(), &mut holder.clone());
    }
    let ans = holder.borrow().right.clone();
    ans
  }

  pub fn dfs(node: Rc<RefCell<TreeNode>>, curr: &mut Rc<RefCell<TreeNode>>) {
    if let Some(left) = node.borrow_mut().left.take() {
      Self::dfs(left, curr);
    }

    curr.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))));
    let new = curr.borrow().right.as_ref().unwrap().clone();
    *curr = new;

    if let Some(right) = node.borrow_mut().right.take() {
      Self::dfs(right, curr);
    }
  }
}