# You are given the root node of a binary search tree (BST) and a value to insert into the tree. Return the root node of the BST after the insertion. It is guaranteed that the new value does not exist in the original BST.
# Notice that there may exist multiple valid ways for the insertion, as long as the tree remains a BST after insertion. You can return any of them.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def insertIntoBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        if not root:
            root = TreeNode(val)
            return root
        runner = root
        while True:
            if val < runner.val:
                if runner.left == None:
                    runner.left = TreeNode(val)
                    return root
                runner = runner.left
            else:
                if runner.right == None:
                    runner.right = TreeNode(val)
                    return root
                runner = runner.right
