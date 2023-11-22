# Given a 0-indexed integer array nums, return the number of distinct quadruplets (a, b, c, d) such that:
    # nums[a] + nums[b] + nums[c] == nums[d], and
    # a < b < c < d

from typing import List


class Solution:
    def countQuadruplets(self, nums: List[int]) -> int:
        n = len(nums)
        count = 0
        for a in range(n-3):
            for b in range(a+1, n-2):
                for c in range(b+1, n-1):
                    for d in range(c+1, n):
                        if nums[a] + nums[b] + nums[c] == nums[d]:
                            count += 1
        return count

s = Solution()
print(s.countQuadruplets([1, 2, 3, 6]))
print(s.countQuadruplets([3, 3, 6, 4, 5]))
print(s.countQuadruplets([1, 1, 1, 3, 5]))
print(s.countQuadruplets([1, 0, 1, 0, 1, 0, 1]))