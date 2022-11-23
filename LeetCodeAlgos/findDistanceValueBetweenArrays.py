# Given two integer arrays arr1 and arr2, and the integer d, return the distance value between the two arrays.
# The distance value is defined as the number of elements arr1[i] such that there is not any element arr2[j] where |arr1[i]-arr2[j]| <= d.


from typing import List


class Solution:
    def findTheDistanceValue(self, arr1: List[int], arr2: List[int], d: int) -> int:
        res = 0
        for i in arr1:
            for j in arr2:
                if abs(i-j) <= d:
                    res -= 1
                    break
            res+=1
        return res

s = Solution()
print(s.findTheDistanceValue([4,5,8], [10,9,1,8], 2))
print(s.findTheDistanceValue([1,4,2,3],[-4,-3,6,10,20,30], 3))
print(s.findTheDistanceValue([2,1,100,3], [-5,-2,10,-3,7], 6))