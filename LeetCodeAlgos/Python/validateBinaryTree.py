# Given the root of a binary tree, determine if it is a valid binary search tree (BST).
#A valid BST is defined as follows:
# The left subtree of a node contains only nodes with keys less than the node's key.
# The right subtree of a node contains only nodes with keys greater than the node's key.
# Both the left and right subtrees must also be binary search trees.



# Definition for a binary tree node.
import math
from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:

        def validate(node, low=-math.inf, high=math.inf):
            if not node:
                return True
            
            if node.val <= low or node.val >= high:
                return False

            return (validate(node.left, low, node.val) and validate(node.right, node.val, high))

        return validate(root);

x = TreeNode(2, TreeNode(1), TreeNode(3))
y = TreeNode(5, TreeNode(1), TreeNode(4, TreeNode(3), TreeNode(6)))
z = TreeNode(32, TreeNode(26, TreeNode(19)), TreeNode(47))

s= Solution()
print(s.isValidBST(x))
print(s.isValidBST(y))
