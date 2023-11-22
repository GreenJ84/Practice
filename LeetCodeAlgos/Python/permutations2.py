# Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.

from typing import List


class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        ans = []
        n = len(nums)

        def buildPerm(_curr, _nums):
            for i in range(len(_nums)):
                x = _curr[:] + [_nums[i]]
                if len(x) == n:
                    if x not in ans:
                        ans.append(x) 
                elif len(x) < n:
                    avail = _nums[:i]+_nums[i+1:]
                    buildPerm(x, avail)
        buildPerm([], nums)
        return ans



s = Solution()
print(s.permuteUnique([1,1,2]))
print(s.permuteUnique([1,2,3]))