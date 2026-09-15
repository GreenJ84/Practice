// You are given the root of a complete binary tree.

// A node x is called dominant if its value is equal to the maximum value among all nodes in the subtree rooted at x.

// Return the number of dominant nodes in the tree.

// Constraints:
// The number of nodes in the tree is in the range [1, 105].
// 1 <= Node.val <= 109
// The tree is guaranteed to be a complete binary tree.

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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_dominant_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 0;
        fn traverse(node: &Rc<RefCell<TreeNode>>, count: &mut i32) -> i32 {
            let node = node.borrow();
            let mut max = 0;
            if let Some(left) = node.left.as_ref() {
                max = max.max(traverse(left, count));
            }
            if let Some(right) = node.right.as_ref() {
                max = max.max(traverse(right, count));
            }
            if node.val >= max {
                *count += 1;
                return node.val;
            }
            max
        }
        traverse(root.as_ref().unwrap(), &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::count_dominant_nodes(root), 5);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::count_dominant_nodes(root), 4);
    }
}
