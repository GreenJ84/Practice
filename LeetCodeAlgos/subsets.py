# Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
# The solution set must not contain duplicate subsets. Return the solution in any order.

from typing import List


class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        ans = [[]]

        def buildSet(_curr, _nums):
            for i in range(len(_nums)):
                x = _curr[:] + [_nums[i]]
                if x not in ans:
                    ans.append(sorted(x))
                buildSet(x, _nums[i+1:])
        buildSet([], nums)
        return ans

s = Solution()
print(s.subsets([1,2,3]))
print(s.subsets([0]))