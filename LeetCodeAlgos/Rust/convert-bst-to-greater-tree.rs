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


struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = &root {
          Self::dfs(Rc::clone(root), &mut 0i32);
        }
        root
    }

    fn dfs(node: Rc<RefCell<TreeNode>>, running_sum: &mut i32) {
      let mut node  = node.borrow_mut();
      if let Some(right) = &node.right {
        Self::dfs(Rc::clone(right), running_sum);
      }
      node.val += *running_sum;
      *running_sum = node.val;
      if let Some(left) = &node.left {
        Self::dfs(Rc::clone(left), running_sum);
      }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn build_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
      return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = vec![root.clone()];
    let mut i = 1;

    while !queue.is_empty() && i < vals.len() {
      let node = queue.remove(0);
      let mut n = node.borrow_mut();

      if i < vals.len() {
        if let Some(val) = vals[i] {
          let left = Rc::new(RefCell::new(TreeNode::new(val)));
          n.left = Some(left.clone());
          queue.push(left);
        }
        i += 1;
      }

      if i < vals.len() {
        if let Some(val) = vals[i] {
          let right = Rc::new(RefCell::new(TreeNode::new(val)));
          n.right = Some(right.clone());
          queue.push(right);
        }
        i += 1;
      }
    }
    Some(root)
  }

  fn compare_trees(node1: &Option<Rc<RefCell<TreeNode>>>, node2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (node1, node2) {
      (None, None) => true,
      (Some(n1), Some(n2)) => {
        let n1 = n1.borrow();
        let n2 = n2.borrow();
        n1.val == n2.val
          && compare_trees(&n1.left, &n2.left)
          && compare_trees(&n1.right, &n2.right)
      }
      _ => false,
    }
  }

  #[test]
  fn test_example_1() {
    let root = build_tree(vec![Some(4), Some(1), Some(6), Some(0), Some(2), Some(5), Some(7), None, None, None, Some(3), None, None, None, Some(8)]);
    let result = Solution::convert_bst(root);
    assert!(compare_trees(&result, &build_tree(vec![Some(30), Some(36), Some(21), Some(36), Some(35), Some(26), Some(15), None, None, None, Some(33), None, None, None, Some(8)])));
  }

  #[test]
  fn test_example_2() {
    let root = build_tree(vec![Some(0), None, Some(1)]);
    let result = Solution::convert_bst(root);
    assert!(compare_trees(&result, &build_tree(vec![Some(1), None, Some(1)])));
  }

  #[test]
  fn test_single_node() {
    let root = build_tree(vec![Some(5)]);
    let result = Solution::convert_bst(root);
    assert!(compare_trees(&result, &build_tree(vec![Some(5)])));
  }

  #[test]
  fn test_empty_tree() {
    let result = Solution::convert_bst(None);
    assert!(compare_trees(&result, &None));
  }

  #[test]
  fn test_all_left_skewed() {
    let root = build_tree(vec![Some(3), Some(2), None, Some(1), None, None, None]);
    let result = Solution::convert_bst(root);
    assert!(compare_trees(&result, &build_tree(vec![Some(3), Some(5), None, Some(6), None, None, None])));
  }

  #[test]
  fn test_all_right_skewed() {
    let root = build_tree(vec![Some(1), None, Some(2), None, Some(3)]);
    let result = Solution::convert_bst(root);
    assert!(compare_trees(&result, &build_tree(vec![Some(6), None, Some(5), None, Some(3)])));
  }

  #[test]
  fn test_negative_values() {
    let root = build_tree(vec![Some(0), Some(-5), Some(5)]);
    let result = Solution::convert_bst(root);
    assert!(compare_trees(&result, &build_tree(vec![Some(5), Some(0), Some(5)])));
  }
}