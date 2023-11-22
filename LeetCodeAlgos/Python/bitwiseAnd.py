# Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.


class Solution:
    def rangeBitwiseAnd(self, left: int, right: int) -> int:
        shift = 0
        while left < right:
            left >>= 1
            right >>= 1
            shift += 1
        return left << shift


# import math
# class Solution:
#     def rangeBitwiseAnd(self, left: int, right: int) -> int:
#         if left == right or left == 0: return left

#         l,r = math.floor(math.log(left, 2)), math.floor(math.log(right, 2))
#         if l!=r: return 0

#         for num in range(left+1, right+1):
#             left = left & num

#         return left
    
s = Solution()
print(s.rangeBitwiseAnd(5, 7))
print(s.rangeBitwiseAnd(0, 0))
print(s.rangeBitwiseAnd(1, 2147483646))