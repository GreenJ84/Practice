# Given two binary trees original and cloned and given a reference to a node target in the original tree.
# The cloned tree is a copy of the original tree.
# Return a reference to the same node in the cloned tree.
# Note that you are not allowed to change any of the two trees or the target node and the answer must be a reference to a node in the cloned tree.

from typing import List


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

class Solution:
    def getTargetCopy(self, original: TreeNode, cloned: TreeNode, target: TreeNode) -> TreeNode:
        curr: List[TreeNode] = [cloned]
        next: List[TreeNode] = []
        while curr:
            for node in curr:
                if node.val == target.val:
                    return node
                if node.left:
                    next.append(node.left)
                if node.right:
                    next.append(node.right)
            curr, next = next, []