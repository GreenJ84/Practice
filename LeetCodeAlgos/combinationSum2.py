# Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.
# Each number in candidates may only be used once in the combination.
# Note: The solution set must not contain duplicate combinations.

from typing import List


class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        ans = []
        if sum(candidates) < target:
            return ans
        def buildSum(curr, avail):
            if len(avail) < 1: return
            for idx, candidate in enumerate(avail):
                x = curr[:] + [candidate]
                attempt = sorted(x)
                if sum(x) == target and attempt not in ans:
                    ans.append(attempt)
                elif sum(x) < target:
                    buildSum(x, avail[:idx]+avail[idx+1:])
        buildSum([], candidates)
        return ans

s = Solution()
print(s.combinationSum2([10,1,2,7,6,1,5], 8))
print(s.combinationSum2([2,5,2,1,2], 5))