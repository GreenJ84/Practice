# Given an n-ary tree, return the level order traversal of its nodes' values.
# Nary-Tree input serialization is represented in their level order traversal, each group of children is separated by the null value (See examples).

from typing import List


class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children

class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        res = []
        lvl, nxt = [root], []

        while lvl:
            for i in lvl:
                if i.children:
                    nxt = nxt[:] + [j for j in i.children]
            res.append([i.val for i in lvl])
            lvl, nxt = nxt, []
        return res

s = Solution()
print(s.levelOrder(Node(1, [Node(2), Node(3, [Node(5), Node(6)]), Node(4)])))