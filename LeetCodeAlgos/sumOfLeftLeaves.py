# Given the root of a binary tree, return the sum of all left leaves.
# A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
        res = 0
        if not root: return res
        if not root.left and not root.right: return root.val

        first, next = [root], []
        while first:
            for n in first:
                if n.left:
                    x = n.left
                    if not x.left and not x.right:
                        res += n.left.val
                    else:
                        next.append(n.left)
                if n.right: 
                    next.append(n.right)
            first, next = next, []
        return res

s = Solution()
print(s.sumOfLeftLeaves(TreeNode( 3,  TreeNode( 9 ), TreeNode( 20, TreeNode( 15 ), TreeNode( 7 )))))
print(s.sumOfLeftLeaves(TreeNode( 3 )))