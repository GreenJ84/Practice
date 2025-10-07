// Given the root of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original BST is changed to the original key plus the sum of all keys greater than the original key in BST.

// As a reminder, a binary search tree is a tree that satisfies these constraints:

// The left subtree of a node contains only nodes with keys less than the node's key.
// The right subtree of a node contains only nodes with keys greater than the node's key.
// Both the left and right subtrees must also be binary search trees.

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

struct Solution{}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() { return root; }
    let mut run_sum = 0;
    Self::right_dfs(&mut root.clone().unwrap(), &mut run_sum);
    root
  }

  pub fn right_dfs(node: &mut Rc<RefCell<TreeNode>>, run: &mut i32){
    let mut node = node.borrow_mut();
    if let Some(right) = &mut node.right {
      Self::right_dfs(right, run);
    }
    node.val += *run;
    *run = node.val;
    if let Some(left) = &mut node.left {
      Self::right_dfs(left, run);
    }
  }
}