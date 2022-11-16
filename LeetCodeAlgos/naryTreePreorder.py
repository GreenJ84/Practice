# Given the root of an n-ary tree, return the preorder traversal of its nodes' values.
# Nary-Tree input serialization is represented in their level order traversal. Each group of children is separated by the null value.

from typing import List


class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution:
    def preorder(self, root: Node) -> List[int]:
        if not root.val:
            return []
        res = [root.val]
        for child in root.children:
            res.append(child.val)
            if child.children:
                res = checkingNode(child, res)
        return res

def checkingNode(node, res):
    for child in node.val:
        res.append(child.val)
        if child.children:
            res = checkingNode(child, res)
    return res
