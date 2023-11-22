# Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes with a value in the inclusive range [low, high].

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def rangeSumBST(self, root: Optional[TreeNode], low: int, high: int) -> int:
        ans = 0
        ans = search(root, ans, low, high)
        return ans
def search(node, ans, low, high):
    if node:
        if low <= node.val <= high:
            ans += node.val
        if low < node.val:
            ans = search(node.left, ans, low, high)
        if node.val < high:
            ans = search(node.right, ans, low, high)
        return ans
