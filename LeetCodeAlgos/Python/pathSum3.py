# Given the root of a binary tree and an integer targetSum, return the number of paths where the sum of the values along the path equals targetSum.
# The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).

from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    ans = 0
    def dps(self, root, curr, target):
        if not root: return
        curr += root.val
        if curr  == target:
            self.ans+=1
        if root.left:
            self.dps(root.left, curr, target)
        if root.right:
            self.dps(root.right, curr, target)

    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        if not root: return 0
        self.dps(root, 0, targetSum)
        if root.left:
            self.pathSum(root.left, targetSum)
        if root.right:
            self.pathSum(root.right, targetSum)
        return self.ans

s = Solution()
print(s.pathSum(TreeNode(10, TreeNode(5, TreeNode(3, TreeNode(3), TreeNode(-1)), TreeNode(2, None, TreeNode(1))), TreeNode(-3, None, TreeNode(11))), 8))
print(s.pathSum(None, 1))
