// You are given a tree with n nodes numbered from 0 to n - 1 in the form of a parent array parent where parent[i] is the parent of the ith node. The root of the tree is node 0, so parent[0] = -1 since it has no parent. You want to design a data structure that allows users to lock, unlock, and upgrade nodes in the tree.

// The data structure should support the following functions:

// Lock: Locks the given node for the given user and prevents other users from locking the same node. You may only lock a node using this function if the node is unlocked.
// Unlock: Unlocks the given node for the given user. You may only unlock a node using this function if it is currently locked by the same user.
// Upgrade: Locks the given node for the given user and unlocks all of its descendants regardless of who locked it. You may only upgrade a node if all 3 conditions are true:
// The node is unlocked,
// It has at least one locked descendant (by any user), and
// It does not have any locked ancestors.
// Implement the LockingTree class:

// LockingTree(int[] parent) initializes the data structure with the parent array.
// lock(int num, int user) returns true if it is possible for the user with id user to lock the node num, or false otherwise. If it is possible, the node num will become locked by the user with id user.
// unlock(int num, int user) returns true if it is possible for the user with id user to unlock the node num, or false otherwise. If it is possible, the node num will become unlocked.
// upgrade(int num, int user) returns true if it is possible for the user with id user to upgrade the node num, or false otherwise. If it is possible, the node num will be upgraded.

// Constraints:
// n == parent.length
// 2 <= n <= 2000
// 0 <= parent[i] <= n - 1 for i != 0
// parent[0] == -1
// 0 <= num <= n - 1
// 1 <= user <= 10^4
// parent represents a valid tree.
// At most 2000 calls in total will be made to lock, unlock, and upgrade.
struct Node {
    parent: isize,
    locked: Option<usize>,
    children: Vec<usize>,
}
struct LockingTree {
    nodes: Vec<Node>,
}

use std::collections::VecDeque;
impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let mut nodes: Vec<Node> = parent
            .iter()
            .map(|&p| Node {
                parent: p as isize,
                locked: None,
                children: Vec::new(),
            })
            .collect();
        for (i, p) in parent.into_iter().enumerate() {
            if p >= 0 {
                nodes[p as usize].children.push(i);
            }
        }
        Self { nodes }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        match self.nodes[num as usize].locked {
            None => {
                self.nodes[num as usize].locked = Some(user as usize);
                true
            }
            Some(_) => false,
        }
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        match self.nodes[num as usize].locked.as_ref() {
            None => false,
            Some(&u) => {
                if u == user as usize {
                    self.nodes[num as usize].locked = None;
                    return true;
                }
                false
            }
        }
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        // Condition 1
        if self.nodes[num as usize].locked.is_some() {
            return false;
        }
        // Condition 3
        let mut i = self.nodes[num as usize].parent;
        while i != -1 {
            if self.nodes[i as usize].locked.is_some() {
                return false;
            }
            i = self.nodes[i as usize].parent;
        }
        // Condition 2
        let mut unlocked = false;
        let mut desc: VecDeque<usize> = VecDeque::from(
            self.nodes[num as usize]
                .children
                .iter()
                .copied()
                .collect::<Vec<usize>>(),
        );
        while !desc.is_empty() {
            let d: usize = desc.pop_front().unwrap();
            if self.nodes[d].locked.is_some() {
                self.nodes[d].locked = None;
                if !unlocked {
                    unlocked = true;
                }
            }
            desc.extend(self.nodes[d].children.iter().copied());
        }
        if unlocked {
            self.nodes[num as usize].locked = Some(user as usize);
        }
        unlocked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let parent = vec![-1, 0, 0, 1, 1, 2, 2];
        let mut obj = LockingTree::new(parent);
        assert_eq!(obj.lock(2, 2), true);
        assert_eq!(obj.unlock(2, 3), false);
        assert_eq!(obj.unlock(2, 2), true);
        assert_eq!(obj.lock(4, 5), true);
        assert_eq!(obj.upgrade(0, 1), true);
        assert_eq!(obj.lock(0, 1), false);
    }
}

// Example 1:

// Input
// ["LockingTree", "lock", "unlock", "unlock", "lock", "upgrade", "lock"]
// [[[-1, 0, 0, 1, 1, 2, 2]], [2, 2], [2, 3], [2, 2], [4, 5], [0, 1], [0, 1]]
// Output
// [null, true, false, true, true, true, false]

// Explanation
// LockingTree lockingTree = new LockingTree([-1, 0, 0, 1, 1, 2, 2]);
// lockingTree.lock(2, 2);    // return true because node 2 is unlocked.
//                            // Node 2 will now be locked by user 2.
// lockingTree.unlock(2, 3);  // return false because user 3 cannot unlock a node locked by user 2.
// lockingTree.unlock(2, 2);  // return true because node 2 was previously locked by user 2.
//                            // Node 2 will now be unlocked.
// lockingTree.lock(4, 5);    // return true because node 4 is unlocked.
//                            // Node 4 will now be locked by user 5.
// lockingTree.upgrade(0, 1); // return true because node 0 is unlocked and has at least one locked descendant (node 4).
//                            // Node 0 will now be locked by user 1 and node 4 will now be unlocked.
// lockingTree.lock(0, 1);    // return false because node 0 is already locked.
