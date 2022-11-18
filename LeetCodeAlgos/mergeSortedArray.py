# You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
# Merge nums1 and nums2 into a single array sorted in non-decreasing order.
# The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

from typing import List


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        j = 0
        for i in range(m, len(nums1)):
            nums1[i] = nums2[j]
            j+=1
        nums1.sort()
        print(nums1)

s = Solution()
s.merge([1,2,3,0,0,0], 3, [2,5,6], 3)
s.merge([1], 1, [], 0)
s.merge([0], 0, [1], 1)


# class Solution:
#     def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        # nums1.sort(), nums2.sort()
        # x, y, i = n, 0, 0
        # while(x < len(nums1) and y < n):
        #     if nums1[x] > nums2[y]:
        #         nums1[i] = nums2[y]
        #         y+=1
        #         i+=1
        #     else: 
        #         nums1[i] = nums1[x]
        #         x+=1
        #         i+=1
        # if y != n:
        #     while(i < len(nums1)):
        #         nums1[i] = nums2[y]
        #         y+= 1
        #         i+=1
        # print(nums1)