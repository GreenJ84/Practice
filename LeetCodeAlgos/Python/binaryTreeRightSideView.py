# Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        if not root: return res

        lvl, nxt = [root], []
        while lvl:
            for i in lvl:
                if i.left: nxt.append(i.left)
                if i.right: nxt.append(i.right)
            res.append(lvl[-1].val)
            lvl, nxt = nxt, []
        return res

s = Solution()
print(s.rightSideView(TreeNode(1, TreeNode(2, None, TreeNode(5)), TreeNode(3, None, TreeNode(4)))))
print(s.rightSideView(TreeNode(1, None, TreeNode(3))))
print(s.rightSideView(None))
print(s.rightSideView(TreeNode(1, TreeNode(2), TreeNode(3))))