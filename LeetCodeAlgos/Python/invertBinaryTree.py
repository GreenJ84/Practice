# Given the root of a binary tree, invert the tree, and return its root.


from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root: return None
        first, next = [root], []
        while first:
            for x in first:
                if x.left:
                    next.append(x.left)
                if x.right:
                    next.append(x.right)
                x.left, x.right = x.right, x.left
            first, next = next, []
        return root

s = Solution()
# print(s.invertTree(TreeNode( 4, TreeNode( 2, TreeNode( 1 ), TreeNode( 3 )), TreeNode( 7, TreeNode( 6 ), TreeNode( 9, )))));

# print(s.invertTree(TreeNode( 2, TreeNode( 1 ), TreeNode( 3 ))));

print(s.invertTree(TreeNode( 1, None, TreeNode( 2 ))));

# print(s.invertTree([]));