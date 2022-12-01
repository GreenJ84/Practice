# You are given two non-increasing 0-indexed integer arrays nums1​​​​​​ and nums2​​​​​​.
# A pair of indices (i, j), where 0 <= i < nums1.length and 0 <= j < nums2.length, is valid if both i <= j and nums1[i] <= nums2[j]. The distance of the pair is j - i​​​​.
# Return the maximum distance of any valid pair (i, j). If there are no valid pairs, return 0.
    # An array arr is non-increasing if arr[i-1] >= arr[i] for every 1 <= i < arr.length.

from typing import List


class Solution:
    def maxDistance(self, nums1: List[int], nums2: List[int]) -> int:
        res = 0
        x, y = 0, 0
        while x<len(nums1) and y<len(nums2):
            if nums1[x] > nums2[y]:
                x+=1
            else:
                res = max(res, y-x)
                y+=1
        return res


s = Solution()
print(s.maxDistance([55,30,5,4,2],[100,20,10,10,5]))
print(s.maxDistance([2,2,2],[10,10,1]))
print(s.maxDistance([30,29,19,5],[25,25,25,25,25]))