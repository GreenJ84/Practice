# Given the root of a binary tree, return its maximum depth.
# A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        lvl = 0
        if not root:
            return lvl

        first, next = [root],[]
        while first:
            lvl+=1
            for n in first:
                if n.left: next.append(n.left)
                if n.right: next.append(n.right)
            
            first, next = next, []
        return lvl

s = Solution()
print(s.maxDepth(TreeNode(3, TreeNode( 9 ), TreeNode( 20, TreeNode( 15 ), TreeNode( 7 )))))
print(s.maxDepth(TreeNode( 1, None, TreeNode( 2 ))))