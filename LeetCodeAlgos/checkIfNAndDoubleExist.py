# Given an array arr of integers, check if there exist two indices i and j such that :
    # i != j
    # 0 <= i, j < arr.length
    # arr[i] == 2 * arr[j]

from typing import List


class Solution:
    def checkIfExist(self, arr: List[int]) -> bool:
        for i in range(len(arr)):
            if arr[i]*2 in arr and arr.index(arr[i]*2) != i:
                return True
        return False

s = Solution()
print(s.checkIfExist([-2,0,10,-19,4,6,-8]))
