# Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to NULL.
# Initially, all next pointers are set to NULL.

class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


class Solution:
    def connect(self, root: 'Node') -> 'Node':
        if not root:
            return root
        lvl, nxt = [root], []
        while lvl:
            nodeLen = len(lvl)
            for n in range(nodeLen):
                if n+1 < nodeLen:
                    lvl[n].next = lvl[n+1]
                if lvl[n].left:
                    nxt.append(lvl[n].left)
                if lvl[n].right:
                    nxt.append(lvl[n].right)
            lvl, nxt = nxt, []
        return root
        
s = Solution()
print(s.connect(
    Node(1, Node(2, Node(4), Node(5)), Node(3, None, Node(7)))
))
print(s.connect(
    Node(None)
))