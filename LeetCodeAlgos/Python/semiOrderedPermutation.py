# You are given a 0-indexed permutation of n integers nums.

# A permutation is called semi-ordered if the first number equals 1 and the last number equals n. You can perform the below operation as many times as you want until you make nums a semi-ordered permutation:

# Pick two adjacent elements in nums, then swap them.
# Return the minimum number of operations to make nums a semi-ordered permutation.

# A permutation is a sequence of integers from 1 to n of length n containing each number exactly once.

from typing import List


class Solution:
    def semiOrderedPermutation(self, nums: List[int]) -> int:
        n = len(nums)
        left = right = -1
        for i in range(len(nums)):
            rPoint = n - 1 - i
            if left < 0:
                if nums[i] == 1:
                    left = i
                elif nums[rPoint] == 1:
                    left = rPoint
            if right < 0:
                if nums[rPoint] == n:
                    right = rPoint
                elif nums[i] == n:
                    right = i
            # print(i, rPoint, left, right)
            if left >= 0 and right >= 0:
                break

        right = (n - 1) - right if left < right else (n - 1) - right - 1
        # print(left, right)
        return left + right
    
s = Solution()
print(s.semiOrderedPermutation([2,1,4,3]))
print(s.semiOrderedPermutation([2,4,1,3]))
print(s.semiOrderedPermutation([1,3,4,2,5]))
