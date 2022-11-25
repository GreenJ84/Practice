# Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
# If target is not found in the array, return [-1, -1].
#! You must write an algorithm with O(log n) runtime complexity.

from typing import List


class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        if target not in nums:
            return [-1,-1]
        else:
            res = []
            res.append(nums.index(target))
            for i in range(res[0]+1, len(nums)):
                if nums[i] != target:
                    res.append(i-1)
                    break
            if i == len(nums):
                res.append(len(nums)-1)
        return res

s = Solution()
print(s.searchRange([5,7,7,8,8,10],8))
print(s.searchRange([5,7,7,8,8,10],6))
print(s.searchRange([], 0))