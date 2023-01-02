# Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.

from typing import List


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        if len(nums) == 1:
            return [[nums[0]]]
        def build(s):
            if len(s) == len(nums):
                ans.append(s)
            else:
                for num in nums:
                    if num not in s:
                        build(s[:]+[num])
        ans = []
        build([])
        return ans

s = Solution()
print(s.permute([1,2,3]))
print(s.permute([0,1]))
print(s.permute([1]))