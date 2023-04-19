# You are given the root of a binary tree.

# A ZigZag path for a binary tree is defined as follow:

# Choose any node in the binary tree and a direction (right or left).
# If the current direction is right, move to the right child of the current node; otherwise, move to the left child.
# Change the direction from right to left or from left to right.
# Repeat the second and third steps until you can't move in the tree.
# Zigzag length is defined as the number of nodes visited - 1. (A single node has a length of 0).

# Return the longest ZigZag path contained in that tree.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def longestZigZag(self, root: Optional[TreeNode]) -> int:
        if not root or (not root.left and not root.right): 
            return 0
        pass

s = Solution()
print(s.longestZigZag(
    TreeNode(1, 
        None, 
        TreeNode(1, 
            TreeNode(1), 
            TreeNode(1, 
                TreeNode(1,
                    None,
                    TreeNode(1, None, TreeNode(1))
                ), 
                TreeNode(1)
            )
        )
    )
))
print(s.longestZigZag(
    TreeNode(1,
        TreeNode(1,
            None,
            TreeNode(1,
                TreeNode(1,
                    None,
                    TreeNode(1),
                ),
                TreeNode(1)
            )
        ),
        TreeNode(1)
    )
))
print(s.longestZigZag(TreeNode(1)))
print(s.longestZigZag(None))