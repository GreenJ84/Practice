# Given a binary tree root and a linked list with head as the first node. 
# Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.
# In this context downward path means a path that starts at some node and goes downwards.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isSubPath(self, head: Optional[ListNode], root: Optional[TreeNode]) -> bool:
        def dfs(hd,node):
            if not hd.next :
                return hd.val==node.val
            if hd.val==node.val:
                if node.left and node.right:
                    return dfs(hd.next,node.left) or dfs(hd.next,node.right)
                elif node.left:
                    return dfs(hd.next,node.left) 
                elif node.right :
                    return dfs(hd.next,node.right)
                else:
                    return False
            else:
                return False
        stk=[root]
        while stk:
            temp=stk.pop()
            if dfs(head,temp):
                return True
            if temp.left:
                stk.append(temp.left)
            if temp.right:
                stk.append(temp.right)
        return False
        
# class Solution:
#     def isSubPath(self, head: Optional[ListNode], root: Optional[TreeNode]) -> bool:
#         lvl, nxt = [root], []
#         while lvl:
#             start = 0
#             for i in lvl:
#                 if i == head:
#                     start = i
#                     nxt = []
#                     break
#                 else:
#                     if i.left:
#                         nxt.append(i.left)
#                     if i.right: 
#                         nxt.append(i.right)
#             lvl, nxt = nxt, []
#         runner = head
#         while runner:
#             if runner.next == start.left:
#                 runner = runner.next
#                 start = start.left
#             elif runner.next==start.right:
#                 runner = runner.next
#                 start = start.right
#             else: return False
#         return True

s = Solution()
print(s.isSubPath(ListNode( 4, ListNode( 2, ListNode( 8))), TreeNode( 1, TreeNode( 4, None, TreeNode( 2, TreeNode( 1 ))), TreeNode( 4, TreeNode( 2, TreeNode( 6 ), TreeNode( 8, TreeNode( 1 ), TreeNode( 3 )))))))