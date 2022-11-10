# You are given an integer array nums. You want to maximize the number of points you get by performing the following operation any number of times:
# Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
# Return the maximum number of points you can earn by applying the above operation some number of times.

from typing import List


class Solution:
    def deleteAndEarn(self, nums: List[int]) -> int:
        points, prev, curr = [0] * 10001, 0, 0
        for num in nums:
            points[num] += num
        for value in points:
            prev, curr = curr, max(prev + value, curr)
        return curr

s = Solution()
# print(s.deleteAndEarn([3,4,2]))
print(s.deleteAndEarn([3,1]))
# print(s.deleteAndEarn([2,2,3,3,3,4]))