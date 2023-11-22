# Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        def build(preO, inO, start, end, dict):
            if start > end or not preO:
                return None
            root_val = preO.pop(0)
            root = TreeNode(root_val)
            idx = dict[root_val]

            root.left = build(preO, inO, start, idx-1, dict)
            root.right = build(preO, inO, idx+1, end, dict)

            return root

        dict = { val: i for i, val in enumerate(inorder) }
        return build(preorder, inorder, 0, len(inorder)-1, dict)