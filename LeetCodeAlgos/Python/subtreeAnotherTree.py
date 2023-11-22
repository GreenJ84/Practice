# Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.
# A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.

from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        if not subRoot: return True
        if not root: return False
        def compTree(node, subNode) -> bool:
            if not node and not subNode:
                return True
            elif not subNode or not node or node.val != subNode.val:
                return False
            else:
                l = compTree(node.left, subNode.left)
                r = compTree(node.right, subNode.right)
                return l and r

        def dps(node):
            if not node:
                return False
            if node.val == subRoot.val:
                if compTree(node, subRoot):
                    return True
            l = dps(node.left)
            r = dps(node.right)
            return l or r

        return dps(root)
    
s = Solution()
print(s.isSubtree(
    TreeNode(3, TreeNode(4, TreeNode(1), TreeNode(2)), TreeNode(5)), 
    TreeNode(4, TreeNode(1), TreeNode(2))
))
print(s.isSubtree(
    TreeNode(3, TreeNode(4, TreeNode(1), TreeNode(2, None, TreeNode(0))), TreeNode(5)),
    TreeNode(4, TreeNode(1), TreeNode(2))
))
