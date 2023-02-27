# Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
# According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”

class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        def dps(node, seen, paths):
            seen.append(node)
            if node == p:
                paths[0] = seen
            if node == q:
                paths[1] = seen
            if node.left:
                dps(node.left, seen[:], paths)
            if node.right:
                dps(node.right, seen[:], paths)
            return paths

        paths = dps(root, [], [None, None])
        i = -1
        while abs(i) <= min(len(paths[0]), len(paths[1])):
            if paths[0][i] in paths[1]:
                return paths[0][i]
            elif paths[1][i] in paths[0]:
                return paths[1][i]
            i -= 1
