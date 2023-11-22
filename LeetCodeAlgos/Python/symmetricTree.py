# Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def dps(self, left, right):
        if not left and not right:
            return True
        if not left or not right or left.val != right.val:
            return False
        else:
            return self.dps(left.left, right.right) and self.dps(left.right, right.left)
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:

        if not root:
            return True
        return self.dps(root.left, root.right)

# class Solution:
#     def isSymmetric(self, root: Optional[TreeNode]) -> bool:
#         first, next, check = [root], [], []
#         while first:
#             for n in first:
#                 if n.left:
#                     next.append(n.left)
#                     check.append(n.left.val)
#                 else:
#                     check.append(101)
#                 if n.right:
#                     next.append(n.right)
#                     check.append(n.right.val)
#                 else: 
#                     check.append(101)
#             if checkArr(check):
#                 first, next, check = next, [], []
#             else: return False
#         return True

# def checkArr(check):
#     x, y = 0, len(check)-1
#     while x<y:
#         if check[x] != check[y]:
#             return False
#         x+=1
#         y-=1
#     return True


s = Solution()
print(s.isSymmetric(TreeNode( 1, TreeNode( 2, TreeNode( 3 ), TreeNode( 4 )), TreeNode( 2, TreeNode( 4 ), TreeNode( 3 )))))

print(s.isSymmetric(TreeNode( 1, TreeNode( 2, None, TreeNode( 3 )), TreeNode( 2, None, TreeNode( 3)))))
