# Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
# You must write an algorithm with O(log n) runtime complexity.

from typing import List


class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if len(nums) == 1:
            return 0 if nums[0] >= target else 1
        start = 0
        end = len(nums)-1
        while start < end:
            if target < nums[start] or target == nums[start]:
                return start
            elif target > nums[end]:
                return end+1
            elif target == nums[end]:
                return end
            x = start+(end-start)//2
            if nums[x] == target:
                return x
            if nums[x] > target:
                end = x-1
            else:
                start = x+1
        if nums[start] > target:
            return start
        else:
            return start+1

s = Solution()
# print(s.searchInsert([1,3], 1))
# print(s.searchInsert([1,2,4,6,7], 3))
print(s.searchInsert([1], 1))
# print(s.searchInsert([1,3,5,6], 5))
# print(s.searchInsert([1,3,5,6], 2))
# print(s.searchInsert([1,3,5,6], 7))
# print(s.searchInsert([1,3,5,6], 4))
