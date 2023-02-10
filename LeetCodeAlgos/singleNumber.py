# Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
# You must implement a solution with a linear runtime complexity and use only constant extra space.

class Solution:
    def singleNumber(self, nums):
        n = 0
        for i in range(len(nums)):
            n ^= nums[i]
        return n

s = Solution()
s.singleNumber([2,2,1])
s.singleNumber([4,1,2,1,2])
s.singleNumber([1])