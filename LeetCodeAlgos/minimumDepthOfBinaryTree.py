# Given a binary tree, find its minimum depth.

# The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

# Note: A leaf is a node with no children.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        

from collections import deque
class Solution:
    def minDepth(self, root: Optional[TreeNode]) -> int:
        if not root: return 0
        if not root.left and not root.right: return 1
        queue = deque([(root, 1)])
        while queue:
            node, depth = queue.popleft()
            if not node.left and not node.right:
                return depth
            if node.left:
                queue.append((node.left, depth + 1))
            if node.right:
                queue.append((node.right, depth + 1))


# class Solution:
#     def minDepth(self, root: Optional[TreeNode]) -> int:
#         if root is None:
#             return 0
#         if root.left is None and root.right is None:
#             return 1
        
#         minLeft = self.minDepth(root.left) if root.left else float('inf')
#         minRight = self.minDepth(root.right) if root.right else float('inf')

#         return 1 + min(minLeft, minRight)