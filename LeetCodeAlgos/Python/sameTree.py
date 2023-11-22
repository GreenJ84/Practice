# Given the roots of two binary trees p and q, write a function to check if they are the same or not.
# Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

import math
from typing import Optional


class TreeNode:
    def __init__(self, val:int=None, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# signigicant Runtime and Memory performance
class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if not p and not q:
            return True
        if not p or not q:
            return False
        if p.val == q.val:
            return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)
        return False

# class Solution:
#     def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
#         if not p and not q:
#             return True
#         if not p or not q:
#             return False

#         lf, ln = [p], []
#         rt, rn = [q], []
#         while lf and rt:
#             if len(lf)!=len(rt):
#                 return False

#             for i in range(len(lf)):
#                 if lf[i].val != rt[i].val:
#                     return False

#                 if lf[i].left and rt[i].left:
#                     ln.append(lf[i].left)
#                     rn.append(rt[i].left)
#                 elif not lf[i].left and not rt[i].left:
#                     pass
#                 else:
#                     return False

#                 if lf[i].right and rt[i].right:
#                     ln.append(lf[i].right)
#                     rn.append(rt[i].right)
#                 elif not lf[i].right and not rt[i].right:
#                     pass
#                 else:
#                     return False
#             lf, ln = ln, []
#             rt, rn = rn, []

#         if lf or rt:
#             return False
#         else:
#             return True 

s = Solution()
print(s.isSameTree(
    TreeNode(1, TreeNode(2), TreeNode(3)), 
    TreeNode(1, TreeNode(2), TreeNode(3))
    ))
print(s.isSameTree(
    TreeNode(1, TreeNode(2)), 
    TreeNode(1, None, TreeNode(2))
    ))
print(s.isSameTree(
    TreeNode(1, TreeNode(2), TreeNode(1)), 
    TreeNode(1, TreeNode(1), TreeNode(2))
    ))
print(s.isSameTree(TreeNode(), TreeNode()))