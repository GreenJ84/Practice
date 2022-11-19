# Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, formed from three of these lengths. If it is impossible to form any triangle of a non-zero area, return 0.

from typing import List

class Solution:
    def largestPerimeter(self, nums: List[int]) -> int:
        nums.sort()
        for i in range(len(nums)-2, 0, -1):
            if nums[i-1]+nums[i]>nums[i+1]:
                return (nums[i-1]+nums[i]+nums[i+1])
        return 0

s = Solution()
print(s.largestPerimeter([2,1,2]))
print(s.largestPerimeter([1,2,1]))
