# Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        n = len(nums)
        if n == 0: return None

        mid = n//2
        return TreeNode(
            nums[mid], 
            self.sortedArrayToBST(nums[:mid]), 
            self.sortedArrayToBST(nums[mid+1:]))

s = Solution()
print(s.sortedArrayToBST([-10,-3,0,5,9]))
print(s.sortedArrayToBST([1,3]))