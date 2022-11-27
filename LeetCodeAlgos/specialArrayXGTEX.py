# You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x.
# Notice that x does not have to be an element in nums.
# Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique.

from typing import List


class Solution:
    def specialArray(self, nums: List[int]) -> int:
        nums.sort(reverse=True)
        gte = 1
        if nums[0] < gte:
            return -1
        for i in range(1, len(nums)):
            if nums[i] == gte:
                return -1
            elif nums[i] < gte+1:
                return gte
            gte+=1
        return gte

s = Solution()
# print(s.specialArray([3,5]))
# print(s.specialArray([0,0]))
# print(s.specialArray([0, 4, 3, 0, 4]))
print(s.specialArray([3, 6, 7, 7, 0]))