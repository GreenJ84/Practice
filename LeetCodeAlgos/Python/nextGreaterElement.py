# The next greater element of some element x in an array is the first greater element that is to the right of x in the same array.
# You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of nums2.
# For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2. If there is no next greater element, then the answer for this query is -1.
# Return an array ans of length nums1.length such that ans[i] is the next greater element as described above.

from typing import List


class Solution:
    def nextGreaterElement(self, nums1: List[int], nums2: List[int]) -> List[int]:
        for i in range(0, len(nums1)):
            maxx = nums1[i]
            x = nums2.index(maxx)
            for j in range(x, len(nums2)):
                if nums2[j] > maxx:
                    nums1[i] = nums2[j]
                    break
            if nums1[i] == maxx:
                nums1[i] = -1
        return nums1

s = Solution()
print(s.nextGreaterElement([4,1,2], [1,3,4,2]))
print(s.nextGreaterElement([2,4], [1,2,3,4]))
