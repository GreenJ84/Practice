# You are given two binary trees root1 and root2.
# Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.
# Return the merged tree.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if root1 == None:
            return root2
        if root2 == None:
            return root1
        root1.val += root2.val;
        root1.left = self.mergeTrees(root1.left, root2.left);
        root1.right = self.mergeTrees(root1.right, root2.right);
        return root1

s = Solution()
print(s.mergeTrees(
    TreeNode( 1, TreeNode( 3, TreeNode( 5 )), TreeNode( 2 )),
    TreeNode( 2, TreeNode( 1, None, TreeNode( 4 )), TreeNode( 3, None, TreeNode( 7 )))
    ))
print(s.mergeTrees(
    TreeNode( 1 ),
    TreeNode( 1, TreeNode( 2 ))
    ))
print(s.mergeTrees(
    TreeNode(),
    TreeNode( 2, TreeNode( 1, None, TreeNode( 4 )), TreeNode( 3, None, TreeNode( 7 )))
    ))
