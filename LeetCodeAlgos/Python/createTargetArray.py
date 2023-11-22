# Given two arrays of integers nums and index. Your task is to create target array under the following rules:

# Initially target array is empty.
# From left to right read nums[i] and index[i], insert at index index[i] the value nums[i] in target array.
# Repeat the previous step until there are no elements to read in nums and index.
# Return the target array.

# It is guaranteed that the insertion operations will be valid.

from typing import List


class Solution:
    def createTargetArray(self, nums: List[int], index: List[int]) -> List[int]:
        n = len(nums)
        ans = [-1] * n
        for i in range(n):
            ans.insert(index[i], nums[i])
            ans.remove(-1)

        return ans
    
s = Solution()
print(s.createTargetArray([0,1,2,3,4], [0,1,2,2,1]))
print(s.createTargetArray([1,2,3,4,0], [0,1,2,3,0]))
print(s.createTargetArray([1], [0]))