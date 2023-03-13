# Given an integer array nums, find a  subarray that has the largest product, and return the product.
# The test cases are generated so that the answer will fit in a 32-bit integer.

from typing import List


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        ans = -float('inf')
        for start in range(0, len(nums)):
            mx = nums[start]
            ans = max(ans, mx)
            for end in range(start+1, len(nums)):
                mx *= nums[end]
                ans = max(ans, mx)
        return ans

s = Solution()
print(s.maxProduct([2,3,-2,4]))
print(s.maxProduct([-2,0,-1]))
print(s.maxProduct([-2]))
print(s.maxProduct([-3, 0, 1, -2]))




