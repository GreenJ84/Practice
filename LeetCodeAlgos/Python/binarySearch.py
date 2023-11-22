# Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
# You must write an algorithm with O(log n) runtime complexity.

from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        start, end = 0, len(nums)-1

        while start <= end:
            x = (start+end)//2
            if target == nums[x]:
                return x
            elif target > nums[x]:
                start = x-1
            else:
                end = x+1
        return -1

s = Solution()
print(s.search([-1,0,3,5,9,12], 9))
print(s.search([-1,0,3,5,9,12], 2))