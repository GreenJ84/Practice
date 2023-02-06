# Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
# We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
# You must solve this problem without using the library's sort function.

from typing import List


class Solution:
    def sortColors(self, nums: List[int]) -> None:
        n = len(nums)
        if n == 1: return nums

        def partition(arr, lf, rt):
            pivot = arr[rt]
            i = lf-1
            for j in range(lf, rt):
                if arr[j] < pivot:
                    i+=1
                    arr[i], arr[j] = arr[j], arr[i]
            arr[i+1], arr[rt] = arr[rt], arr[i+1]
            return i+1

        def quicksort(arr, lf, rt):
            if lf < rt:
                mid = partition(arr, lf, rt)
                quicksort(arr, lf, mid-1)
                quicksort(arr, mid+1, rt)

        quicksort(nums, 0, n-1)
        return nums

s = Solution()
print(s.sortColors([2,0,2,1,1,0]))
print(s.sortColors([2,0,1]))