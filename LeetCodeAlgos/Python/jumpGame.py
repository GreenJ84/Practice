# You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
# Return true if you can reach the last index, or false otherwise.

from typing import List

# 155/170 Cases. Can't efficiently check and get stuck in edge cases
class Solution1:
    def canJump1(self, nums: List[int]) -> bool:
        for i in range(0, len(nums)):
            x = nums[i]

            if i+x >= len(nums)-1 or len(nums) == 1:
                return True

            elif nums[i+x] != 0:
                i += x

            else:
                s = 1
                while s < x:
                    if nums[i+x-s] != 0+s:
                        i += x-s
                        break
                    s += 1
                if s >= x:
                    return False
        return True

class Solution:
    def canJump(self, nums: List[int]) -> bool:
        end = len(nums)-1
        for i in range(len(nums)-2, -1, -1):
            if i+nums[i] >= end:
                end = i
        return end == 0

s = Solution()
print(s.canJump([2,3,1,1,4]))
print(s.canJump([3,2,1,0,4]))
print(s.canJump([0]))