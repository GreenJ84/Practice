// Given the root of a binary tree, return the sum of every tree node's tilt.

// The tilt of a tree node is the absolute difference between the sum of all left subtree node values and all right subtree node values. If a node does not have a left child, then the sum of the left subtree node values is treated as 0. The rule is similar if the node does not have a right child.

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
struct Solution;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.as_ref().map(|n| Self::tree_sum(n).1).unwrap_or(0)
    }

    fn tree_sum(node: &RefCell<TreeNode>) -> (i32, i32) {
        let node = node.borrow();
        let (l_sum, l_tilt) = node
            .left
            .as_ref()
            .map(|n| Self::tree_sum(n))
            .unwrap_or((0, 0));
        let (r_sum, r_tilt) = node
            .right
            .as_ref()
            .map(|n| Self::tree_sum(n))
            .unwrap_or((0, 0));
        (
            l_sum + r_sum + node.val,
            l_tilt + r_tilt + (l_sum - r_sum).abs(),
        )
    }

    pub fn find_tilt1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.as_ref().map(Rc::clone).unwrap();
        let tilt_sum: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
        Self::find_tree_sum(root, Rc::clone(&tilt_sum));
        Rc::unwrap_or_clone(tilt_sum).into_inner()
    }

    fn find_tree_sum(node: Rc<RefCell<TreeNode>>, tilt_sum: Rc<RefCell<i32>>) -> i32 {
        let node = node.borrow();
        let mut left = 0;
        if let Some(l) = node.left.as_ref() {
            left = Self::find_tree_sum(Rc::clone(l), Rc::clone(&tilt_sum));
        }
        let mut right = 0;
        if let Some(r) = node.right.as_ref() {
            right = Self::find_tree_sum(Rc::clone(r), Rc::clone(&tilt_sum));
        }
        {
            *tilt_sum.borrow_mut() += (left - right).abs();
        }
        left + right + node.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(Solution::find_tilt(root), 1);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(Solution::find_tilt(root), 15);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 21,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 14,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
        })));
        assert_eq!(Solution::find_tilt(root), 9);
    }
}

// Example 1:

// Input: root = [1,2,3]
// Output: 1
// Explanation:
// Tilt of node 2 : |0-0| = 0 (no children)
// Tilt of node 3 : |0-0| = 0 (no children)
// Tilt of node 1 : |2-3| = 1 (left subtree is just left child, so sum is 2; right subtree is just right child, so sum is 3)
// Sum of every tilt : 0 + 0 + 1 = 1
// Example 2:

// Input: root = [4,2,9,3,5,null,7]
// Output: 15
// Explanation:
// Tilt of node 3 : |0-0| = 0 (no children)
// Tilt of node 5 : |0-0| = 0 (no children)
// Tilt of node 7 : |0-0| = 0 (no children)
// Tilt of node 2 : |3-5| = 2 (left subtree is just left child, so sum is 3; right subtree is just right child, so sum is 5)
// Tilt of node 9 : |0-7| = 7 (no left child, so sum is 0; right subtree is just right child, so sum is 7)
// Tilt of node 4 : |(3+5+2)-(9+7)| = |10-16| = 6 (left subtree values are 3, 5, and 2, which sums to 10; right subtree values are 9 and 7, which sums to 16)
// Sum of every tilt : 0 + 0 + 0 + 2 + 7 + 6 = 15
// Example 3:

// Input: root = [21,7,14,1,1,2,2,3,3]
// Output: 9

// Constraints:

// The number of nodes in the tree is in the range [0, 104].
// -1000 <= Node.val <= 1000
