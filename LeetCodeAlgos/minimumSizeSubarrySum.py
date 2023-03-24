# Given an array of positive integers nums and a positive integer target, return the minimal length of a 
# subarray
#  whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.

from typing import List


class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        if sum(nums) < target: return 0
        ans, left = len(nums), 0
        sm = 0
        for right, val in enumerate(nums):
            sm += val
            while sm >= target and left <= right:
                ans = min(ans, right - left + 1)
                sm -= nums[left]
                left += 1
            if ans == 1: return ans
        return ans
    
s = Solution()
print(s.minSubArrayLen(7, [2,3,1,2,4,3]))
print(s.minSubArrayLen(4, [1,4,4]))
print(s.minSubArrayLen(11, [1,1,1,1,1,1,1,1]))


