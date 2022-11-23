# An array arr a mountain if the following properties hold:
# There exists some i with 0 < i < arr.length - 1 such that:
    # arr[0] < arr[1] < ... < arr[i - 1] < arr[i] 
    # arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
# Given a mountain array arr, return the index i.
# You must solve it in O(log(arr.length)) time

from typing import List


class Solution:
    def peakIndexInMountainArray(self, arr: List[int]) -> int:
        for i in range(1, len(arr)-1):
            if arr[i-1]>arr[i]>arr[i+1]:
                return i-1
            elif arr[i-1]<arr[i]>arr[i+1]:
                return i
            if arr[-i]>arr[-i-1]>arr[-i-2]:
                return len(arr)-1
            elif arr[-i]<arr[-i-1]>arr[-i-2]:
                return len(arr)-i-1

s = Solution()
print(s.peakIndexInMountainArray([0,1,0]))
print(s.peakIndexInMountainArray([0,2,1,0]))
print(s.peakIndexInMountainArray([0,1,5,12]))
