# Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
# The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
# You must write an algorithm that runs in O(n) time and without using the division operation.

import functools
from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        prod = [1]*n

        fix = 1
        for i in range(n):
            prod[i] = fix
            fix *= nums[i]

        fix = 1
        for i in range(n-1, -1, -1):
            prod[i] *= fix
            fix *= nums[i]
        return prod

s = Solution()
print(s.productExceptSelf([1,2,3,4]))
print(s.productExceptSelf([-1,1,0,-3,3]))