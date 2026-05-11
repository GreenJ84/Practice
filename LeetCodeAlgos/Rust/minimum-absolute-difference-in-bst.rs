// Given the root of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.

// Constraints:
// - The number of nodes in the tree is in the range [2, 104].
// - 0 <= Node.val <= 105

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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nodes = vec![];
        match root.as_ref() {
            Some(node) => {
                let mut bfs = vec![node.clone()];
                while !bfs.is_empty() {
                    let node = bfs.pop().unwrap();
                    let node = node.borrow();
                    nodes.push(node.val);
                    if let Some(left) = node.left.as_ref() {
                        bfs.push(left.clone());
                    }
                    if let Some(right) = node.right.as_ref() {
                        bfs.push(right.clone());
                    }
                }
            }
            None => return 0,
        }
        
        nodes.sort_unstable();
        nodes.windows(2).fold(i32::MAX, |min_diff, pair| {
            min_diff.min((pair[0] - pair[1]).abs())
        })
    }

    // Does not account for the fact that the minimum difference may be between two subtree nodes that are not parent and child
    pub fn _get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_diff = i32::MAX;
        match root {
            Some(node) => {
                let node = node.borrow();
                if let Some(left) = node.left.as_ref() {
                    let min = Self::bfs(left.clone(), &mut min_diff, false);
                    println!("min: {min}");
                    min_diff = min_diff.min((node.val - min).abs());
                    println!("- min_diff: {}", min_diff);
                }
                if let Some(right) = node.right.as_ref() {
                    let max = Self::bfs(right.clone(), &mut min_diff, true);
                    println!("max: {max}");
                    min_diff = min_diff.min((node.val - max).abs());
                    println!("- min_diff: {}", min_diff);
                }
            }
            None => return 0,
        }
        min_diff
    }

    fn bfs(node: Rc<RefCell<TreeNode>>, min_diff: &mut i32, find_min: bool) -> i32 {
        let mut bfs = vec![node];
        let mut find = if find_min { i32::MAX } else { i32::MIN };
        while !bfs.is_empty() {
            let node = bfs.pop().unwrap();
            let node = node.borrow();
            if find_min {
                find = find.min(node.val);
            } else {
                find = find.max(node.val);
            }
            println!("node: {}", node.val);
            if let Some(left) = node.left.as_ref() {
                *min_diff = (*min_diff).min(node.val - left.borrow().val);
                println!(".   left: {}", left.borrow().val);
                bfs.push(left.clone());
            }
            if let Some(right) = node.right.as_ref() {
                *min_diff = (*min_diff).min(right.borrow().val - node.val);
                println!(".   right: {}", right.borrow().val);
                bfs.push(right.clone());
            }
            println!("- min_diff: {}", min_diff);
        }
        find
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 48,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 12,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 49,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(Solution::get_minimum_difference(root), 2);
    }
}
