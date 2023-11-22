# You are given a perfect binary tree where all leaves are on the same level, and every parent has two children.
# Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to NULL.
# Initially, all next pointers are set to NULL.

from typing import Optional


class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next

class Solution:
    def connect(self, root: Optional[Node]) -> Optional[Node]:
        if not root:
            return root
        lvl, next = [root], []
        while lvl:
            for i in range(len(lvl)):
                if i!=len(lvl)-1:
                    lvl[i].next = lvl[i+1]
                else:
                    lvl[i].next = None
                if lvl[i].left:
                    next.append(lvl[i].left)
                if lvl[i].right:
                    next.append(lvl[i].right)
            lvl, next = next, []
        return root

s = Solution()
print(s.connect(
    Node(1, 
        Node(2, Node(4), Node(5)), 
        Node(3, Node(6), Node(7)), 
    )
))
print(s.connect(
    Node()
))