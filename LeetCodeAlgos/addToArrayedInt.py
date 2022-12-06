# The array-form of an integer num is an array representing its digits in left to right order.
# For example, for num = 1321, the array form is [1,3,2,1].
# Given num, the array-form of an integer, and an integer k, return the array-form of the integer num + k.

from typing import List


class Solution:
    def addToArrayForm(self, num: List[int], k: int) -> List[int]:
        # int to List
        nK = []
        while k:
            nK.append(k%10)
            k = k//10
        nK.reverse()

        # Start adding back to front
        res, sum = [], 0
        lenn = -max(len(num), len(nK))

        for i in range(-1, lenn-1, -1):
            if i >= -len(num):
                sum+=num[i]
            if i >= -len(nK):
                sum+=nK[i]
            
            res.append(sum%10)
            sum = sum//10
        
        if sum != 0:
            res.append(sum)
        res.reverse()
        return res

s = Solution()
print(s.addToArrayForm([1,2,0,0], 34))
print(s.addToArrayForm([2,7,4], 181))
print(s.addToArrayForm([2,1,5], 806))