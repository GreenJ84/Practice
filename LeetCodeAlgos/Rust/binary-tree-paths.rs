// Given the root of a binary tree, return all root-to-leaf paths in any order.

// A leaf is a node with no children.

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

use std::rc::Rc;
use std::cell::RefCell;
struct Solution {}
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = Vec::<String>::new();
        if root.is_none() { return ans; }

        Self::dfs(&root.unwrap(), &mut ans, &mut Vec::new());
        ans
    }

    pub fn dfs(node: &Rc<RefCell<TreeNode>>, mut ans: &mut Vec<String>, mut curr_path: &mut Vec<i32>) {
      curr_path.push(node.borrow().val);

      if node.borrow().left.is_none() && node.borrow().right.is_none() {
          ans.push(curr_path
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("->")
          );
          curr_path.pop();
          return;
      }
      if let Some(node) = &node.borrow().left {
        Self::dfs(&node, &mut ans, &mut curr_path);
      }
      if let Some(node) = &node.borrow().right {
        Self::dfs(&node, &mut ans, &mut curr_path);
      }
      curr_path.pop();
    }
}

