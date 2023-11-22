# Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

# Definition for a binary tree node.
from typing import List, Optional


class TreeNode:
    def __init__(self, val: int = 0, left= None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def levelOrder(self, root: TreeNode) -> List[List[int]]:
        if not root:
            return []
        q, nxq, lvl, res = [root], [], [], []

        while q:
            for i in q:
                lvl.append(i.val)
                if i.left:
                    nxq.append(i.left)
                if i.right:
                    nxq.append(i.right)
            res.append(lvl)
            lvl, q, nxq = [], nxq, []
        return res





s = Solution()
