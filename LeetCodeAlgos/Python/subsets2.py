# Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
# The solution set must not contain duplicate subsets. Return the solution in any order.

from typing import List


class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        ans = [[]]
        def buildSubset(curr, avail):
            for num in range(len(avail)):
                x = curr + [avail[num]]
                att = sorted(x)
                if att not in ans:
                    ans.append(att)
                if len(x) != len(nums):
                    buildSubset(x, avail[num+1:])
        buildSubset([], nums)
        return ans
    
s = Solution()
print(s.subsetsWithDup([1,2,2]))
print(s.subsetsWithDup([0]))