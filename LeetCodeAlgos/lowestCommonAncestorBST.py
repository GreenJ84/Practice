# Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.
# According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x, left=None, right=None):
        self.val = x
        self.left = left
        self.right = right

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':

        def find(n, p=p, q=q):
            if n.val == p.val or n.val == q.val:
                x = n
            elif (p.val < n.val) ^ (q.val < n.val):
                x = n
            else:
                if p.val < n.val:
                    x = find(n.left, p, q)
                elif p.val > n.val:
                    x = find(n.right, p, q)
            return x
        return find(root)

x = TreeNode(6, TreeNode(2, TreeNode(0), TreeNode(4, TreeNode(3), TreeNode(5))), TreeNode(8, TreeNode(7), TreeNode(9)))

s = Solution()
# print(s.lowestCommonAncestor(x, 2, 8))
print(s.lowestCommonAncestor(x, TreeNode(2), TreeNode(4)).val)
