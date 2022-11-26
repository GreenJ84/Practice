# Given the root of a binary tree, return the preorder traversal of its nodes' values.

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        if root:
            traverse(root, res)
        return res

def traverse(root, res):
    res.append(root.val)
    if root.left:
        traverse(root.left, res)
    if root.right:
        traverse(root.right, res)
    return res

s = Solution()
print(s.preorderTraversal(TreeNode( 1, None, TreeNode( 2, TreeNode( 3 )))))
print(s.preorderTraversal([]))
print(s.preorderTraversal(TreeNode( 1 )))