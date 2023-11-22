# Given the root of a binary search tree and an integer k, return true if there exist two elements in the BST such that their sum is equal to k, or false otherwise.

from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def findTarget(self, root: Optional[TreeNode], k: int) -> bool:
        if not root.left and not root.right:
            return False
        search, mat, lvl = [], [root], []
        while mat:
            for i in mat:
                search.append(i.val)
                if i.left: lvl.append(i.left)
                if i.right: lvl.append(i.right)
            mat, lvl = lvl, []
        search.sort()
        start, end = 0, len(search)-1
        while start<end:
            if search[start]+search[end]==k:
                return True
            elif search[start]+search[end]<k:
                start+=1
            else:
                end-=1
        return False

s = Solution()
# print(s.findTarget(TreeNode( 5, TreeNode( 3, TreeNode( 2 ), TreeNode( 4 )),TreeNode( 6, None, TreeNode( 7 ))), 28))
print(s.findTarget(TreeNode( 5, TreeNode( 3, TreeNode( 2 ), TreeNode( 4 )),TreeNode( 6, None, TreeNode( 7 ))), 9))
