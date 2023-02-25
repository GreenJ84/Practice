# Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.

from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        res = []
        lvl, nxt = [root], []
        while lvl:
            for i in lvl: 
                if i.left: nxt.append(i.left)
                if i.right: nxt.append(i.right)
                res.append(i.val)
            lvl, nxt = nxt, []
        res.sort()
        return res[k-1]
    
s = Solution()
print(s.kthSmallest(TreeNode(3, TreeNode(1, None, TreeNode(2)), TreeNode(4)), 1))
# print(s.kthSmallest(TreeNode(5, TreeNode(3, TreeNode(2, TreeNode(1)), TreeNode(4)), TreeNode(6)), 3))