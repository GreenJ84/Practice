# A binary tree is uni-valued if every node in the tree has the same value.

# Given the root of a binary tree, return true if the given tree is uni-valued, or false otherwise.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isUnivalTree(self, root: Optional[TreeNode]) -> bool:
        match = root.val
        def bfs(node):
            if not node:
                return True
            elif node.val != match:
                return False
            return bfs(node.left) and bfs(node.right)

        return bfs(root)
    
s = Solution()
first = TreeNode(1, 
                TreeNode(1,
                        TreeNode(1),
                        TreeNode(1)
                ),
                TreeNode(1, 
                        None, 
                        TreeNode(1)
                )
        )
print(s.isUnivalTree(first))

second = TreeNode(2, 
                TreeNode(2,
                        TreeNode(5),
                        TreeNode(2)
                ),
                TreeNode(2
                )
        )
print(s.isUnivalTree(second))