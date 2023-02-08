# Given the root of a binary tree, return the length of the diameter of the tree.
# The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
# The length of a path between two nodes is represented by the number of edges between them.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if not root.left and not root.right: return 0
        rec = []

        def dps(node, dist):
            lf = dps(node.left, 1) if node.left else 0
            rt = dps(node.right, 1) if node.right else 0
            rec.append(lf+rt)
            return dist+max(lf,rt)
        dps(root, 0)
        return max(rec)

s = Solution()
print(s.diameterOfBinaryTree(TreeNode(1,
                                TreeNode(2, 
                                    TreeNode(4),
                                    TreeNode(5)
                                ),
                                TreeNode(3)
                            )))

print(s.diameterOfBinaryTree(TreeNode(1, TreeNode(2))))

