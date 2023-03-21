# A peak element is an element that is strictly greater than its neighbors.
# Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.
# You may imagine that nums[-1] = nums[n] = -∞. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.

from typing import List


class Solution:
    def findPeakElement(self, nums: List[int]) -> int:
        for i in range(len(nums)):
            if (i-1 == -1 or nums[i]>nums[i-1]) and (i+1 == len(nums) or nums[i]>nums[i+1]):
                return i
        return -1
    
s = Solution()
print(s.findPeakElement([1,2,3,1]))
print(s.findPeakElement([1,2,1,3,5,6,4]))
print(s.findPeakElement([1,2,3,5,6,9]))