# Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where the sum of the node values in the path equals targetSum. Each path should be returned as a list of the node values, not node references.
# A root-to-leaf path is a path starting from the root and ending at any leaf node. A leaf is a node with no children.

from typing import List, Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        def runTree(node, currSum, seen, ans):
            currSum += node.val
            seen.append(node.val)
            if not node.left and not node.right:
                if currSum == targetSum:
                    ans.append(seen)
            if node.left:
                ans = runTree(node.left, currSum, seen[:], ans)
            if node.right:
                ans = runTree(node.right, currSum, seen[:], ans)
            return ans

        if not root:
            return []
        return runTree(root, 0, [], [])

s = Solution()
print(s.pathSum(TreeNode(1, TreeNode(2), TreeNode(3)), 5))
print(s.pathSum(TreeNode(1, TreeNode(2)), 0))
# print(s.pathSum(TreeNode(5, TreeNode(4, TreeNode(11, TreeNode(7), TreeNode(2))), TreeNode(8, TreeNode(13), TreeNode(4, TreeNode(5), TreeNode(1)))), 22))
print(s.pathSum(TreeNode(1, TreeNode(2), TreeNode(3)), 4))