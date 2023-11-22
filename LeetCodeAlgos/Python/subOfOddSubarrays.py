#Given an array of positive integers arr, return the sum of all possible odd-length subarrays of arr.
#A subarray is a contiguous subsequence of the array.

from typing import List


class Solution:
    def sumOddLengthSubarrays(self, arr: List[int]) -> int:
        res = 0
        for i in range(1, len(arr)+1, 2):
            x, y = 0, i 
            while y != len(arr)+1:
                set = arr[x:y]
                res += sum(set)
                x+=1
                y+=1
        return res

s = Solution()
print(s.sumOddLengthSubarrays([1,4,2,5,3]))
print(s.sumOddLengthSubarrays([1,2]))
print(s.sumOddLengthSubarrays([10,11,12]))
