// Given the root of a binary tree, flatten the tree into a "linked list":

// The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
// The "linked list" should be in the same order as a pre-order traversal of the binary tree.

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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut tail = None;
        Self::dfs(root, &mut tail);
    }

    fn dfs(node: &mut Option<Rc<RefCell<TreeNode>>>, tail: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            // If there's a tail, set its right to current node
            if let Some(t) = tail {
                t.borrow_mut().right = Some(n.clone());
            }
            // Move tail to current node
            *tail = Some(n.clone());

            // Save subtrees
            let mut left = n.borrow_mut().left.take();
            let mut right = n.borrow_mut().right.take();
            // Set left to None
            n.borrow_mut().left = None;

            // Recurse left first
            Self::dfs(&mut left, tail);
            // Then right
            Self::dfs(&mut right, tail);
        }
    }
}
