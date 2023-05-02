# There is a function signFunc(x) that returns:
# 1 if x is positive.
# -1 if x is negative.
# 0 if x is equal to 0.
# You are given an integer array nums. Let product be the product of all values in the array nums.
# Return signFunc(product).

import functools
from typing import List


class Solution:
    def arraySign(self, nums: List[int]) -> int:
        if 0 in nums:
            return 0
        negs = 0
        for i in nums:
            if i < 0:
                negs += 1
        
        return -1 if negs % 2 == 1 else 1

# class Solution:
#     def arraySign(self, nums: List[int]) -> int:
#         if 0 in nums:
#             return 0
#         return 1 if functools.reduce(lambda x, y: x*y, nums) > 0 else -1


s = Solution()
print(s.arraySign([-1,-2,-3,-4,3,2,1]))
print(s.arraySign([1,5,0,2,-3]))
print(s.arraySign([-1,1,-1,1,-1]))