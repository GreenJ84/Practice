# Given a binary tree, determine if it is height-balanced

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if not root: return True
        if abs(depth(root.left)-depth(root.right))<=1:
            if root.left:
                if not self.isBalanced(root.left):
                    return False
            if root.right:
                if not self.isBalanced(root.right):
                    return False
            return True
        else:
            return False

def depth( node):
    if not node: return 0
    return max(depth(node.left), depth(node.right))+1

s = Solution()
print(s.isBalanced(TreeNode(1, TreeNode(2, TreeNode(3, TreeNode(4))), TreeNode(2, None, TreeNode(3, None, TreeNode(4))))))
