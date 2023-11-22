# Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        all = {}
        for i in nums:
            if i not in all:
                all[i] = 1
            else:
                return True
        return False

s = Solution()
print(s.containsDuplicate([1,2,3,1]))
print(s.containsDuplicate([1,2,3,4]))
print(s.containsDuplicate([1,1,1,3,3,4,3,2,4,2]))