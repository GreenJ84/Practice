# Given the root of a binary tree, return the inorder traversal of its nodes' values.

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        if root:
            traverse(root, res)
        return res

def traverse(root, res):
    if root.left:
        traverse(root.left, res)
    res.append(root.val)
    if root.right:
        traverse(root.right, res)
    return res

s = Solution()
print(s.inorderTraversal(TreeNode( 1, None, TreeNode( 2, TreeNode( 3 )))))
print(s.inorderTraversal([]))
print(s.inorderTraversal(TreeNode( 1 )))