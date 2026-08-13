// Given the root of a binary search tree (BST) with duplicates, return all the mode(s) (i.e., the most frequently occurred element) in it.

// If the tree has more than one mode, return them in any order.

// Assume a BST is defined as follows:

// The left subtree of a node contains only nodes with keys less than or equal to the node's key.
// The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
// Both the left and right subtrees must also be binary search trees.

// Constraints:
// The number of nodes in the tree is in the range [1, 104].
// -105 <= Node.val <= 105

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
use std::collections::HashMap;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut freq = HashMap::<i32, i32>::new();
        if let Some(node) = root {
            Self::add_freq(node, &mut freq);
        } else {
            return vec![];
        }

        let mut freq: Vec<(i32, i32)> = freq.into_iter().collect();
        freq.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let max_freq = freq[0].1;
        freq.into_iter()
            .filter_map(|(val, fr)| if fr == max_freq { Some(val) } else { None })
            .collect()
    }

    fn add_freq(node: Rc<RefCell<TreeNode>>, freq: &mut HashMap<i32, i32>) {
        let node = node.borrow();
        *freq.entry(node.val).or_insert(0) += 1;
        if let Some(left) = node.left.as_ref() {
            Self::add_freq(Rc::clone(left), freq);
        }
        if let Some(right) = node.right.as_ref() {
            Self::add_freq(Rc::clone(right), freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(Solution::find_mode(root), vec![2]);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        assert_eq!(Solution::find_mode(root), vec![0]);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(Solution::find_mode(root), vec![2, 1]);
    }
}
