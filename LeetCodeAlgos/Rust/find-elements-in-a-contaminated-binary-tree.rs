// Given a binary tree with the following rules:

// root.val == 0
// For any treeNode:
// If treeNode.val has a value x and treeNode.left != null, then treeNode.left.val == 2 * x + 1
// If treeNode.val has a value x and treeNode.right != null, then treeNode.right.val == 2 * x + 2
// Now the binary tree is contaminated, which means all treeNode.val have been changed to -1.

// Implement the FindElements class:

// FindElements(TreeNode* root) Initializes the object with a contaminated binary tree and recovers it.
// bool find(int target) Returns true if the target value exists in the recovered binary tree.

// Constraints:
// TreeNode.val == -1
// The height of the binary tree is less than or equal to 20
// The total number of nodes is between [1, 104]
// Total calls of find() is between [1, 104]
// 0 <= target <= 10^6

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
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;
struct FindElements {
    known_values: Vec<i32>,
    boundary: VecDeque<Rc<RefCell<TreeNode>>>,
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        root.as_ref().unwrap().borrow_mut().val = 0;
        Self {
            known_values: vec![0],
            boundary: VecDeque::from([Rc::clone(root.as_ref().unwrap())]),
        }
    }

    fn find(&mut self, target: i32) -> bool {
        match target.cmp(&self.known_values[self.known_values.len() - 1]) {
            Ordering::Equal => true,
            Ordering::Less => self.known_values.binary_search(&target).is_ok(),
            Ordering::Greater => {
                let mut found: Option<bool> = None;
                loop {
                    if let Some(node) = self.boundary.pop_front() {
                        let node = node.borrow();
                        if let Some(left) = node.left.as_ref() {
                            left.borrow_mut().val = node.val * 2 + 1;
                            let val = left.borrow().val;
                            self.known_values.push(val);
                            self.boundary.push_back(Rc::clone(&left));
                            if val == target {
                                found = Some(true);
                            } else if val > target {
                                found = Some(false);
                            }
                        }
                        if let Some(right) = node.right.as_ref() {
                            right.borrow_mut().val = node.val * 2 + 2;
                            let val = right.borrow().val;
                            self.known_values.push(val);
                            self.boundary.push_back(Rc::clone(&right));
                            if found.is_some() {
                                break found.unwrap();
                            } else if val == target {
                                found = Some(true);
                            } else if val > target {
                                found = Some(false);
                            }
                        }
                        if let Some(res) = found {
                            break res;
                        }
                    } else {
                        break false;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: None,
                right: None,
            }))),
        })));
        let mut obj = FindElements::new(root);
        assert_eq!(obj.find(1), false);
        assert_eq!(obj.find(2), true);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: None,
                right: None,
            }))),
        })));
        let mut obj = FindElements::new(root);
        assert_eq!(obj.find(1), true);
        assert_eq!(obj.find(3), true);
        assert_eq!(obj.find(5), false);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let mut obj = FindElements::new(root);
        assert_eq!(obj.find(2), true);
        assert_eq!(obj.find(3), false);
        assert_eq!(obj.find(4), false);
        assert_eq!(obj.find(5), true);
    }
}

// Example 3:

// Input
// ["FindElements","find","find","find","find"]
// [[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
// Output
// [null,true,false,false,true]
// Explanation
// FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
// findElements.find(2); // return True
// findElements.find(3); // return False
// findElements.find(4); // return False
// findElements.find(5); // return True
