# Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.

from typing import List


class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2 == 1: return False

        half_target = total // 2
        sumPossible = [True]+[False] * (half_target)
        
        for num in nums:
            for j in range(half_target, num-1, -1):
                if sumPossible[j]: continue
                sumPossible[j] = sumPossible[j - num]

        return sumPossible[half_target]


s = Solution()
print(s.canPartition([1,5,11,5]))
print(s.canPartition([1,2,3,5]))
print(s.canPartition([2,2,3,5]))