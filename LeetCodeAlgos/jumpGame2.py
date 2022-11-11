# You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
# Each element nums[i] represents the maximum length of a forward jump from index i.
# Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

from typing import List


class Solution:
    def jump(self, nums: List[int]) -> bool:
        end = len(nums)-1
        jumps = 0
        while end != 0:
            check = end-1
            lJ = end
            for i in range(check, -1, -1):
                if i+nums[i] >= end and i < lJ:
                    lJ = i
            end = lJ
            jumps += 1
        return jumps

s = Solution()
print(s.jump([2,3,1,1,4]))
print(s.jump([2,3,0,1,4]))

