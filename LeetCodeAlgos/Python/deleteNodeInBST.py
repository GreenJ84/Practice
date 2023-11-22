# Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
# Basically, the deletion can be divided into two stages:
    # Search for a node to remove.
    # If the node is found, delete the node.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        def delete(node, parent):
            successor = node.right
            while successor and successor.left:
                successor = successor.left

            if not parent:
                if successor:
                    successor.left = node.left
                    return node.right
                else:
                    return node.left
            elif parent.val > node.val:
                if successor:
                    parent.left, successor.left = node.right, node.left
                else:
                    parent.left = node.left
            else:
                if successor:
                    parent.right, successor.left = node.right, node.left
                else:
                    parent.right = node.left
            return root


        runner = root
        parent = None
        while runner and runner.val != key:
            parent = runner
            if runner.val > key:
                if runner.left:
                    runner = runner.left
                else: return root
            else:
                if runner.right:
                    runner = runner.right
                else: return root
        if not runner: return root
        return delete(runner, parent)