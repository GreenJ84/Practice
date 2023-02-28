# Given an integer array nums and an integer k, return the kth largest element in the array.
# Note that it is the kth largest element in the sorted order, not the kth distinct element.
# You must solve it in O(n) time complexity.

from typing import List

class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        nums.sort(reverse=True)
        return nums[k-1]

# class Solution:
#     def findKthLargest(self, nums: List[int], k: int) -> int:
#         hold = nums[:k]
#         for i in range(k, len(nums)):
#             mn = min(hold)
#             if nums[i] <= mn:
#                 continue
#             else:
#                 hold.remove(mn)
#                 hold.append(nums[i])
#         return min(hold)

s = Solution()
print(s.findKthLargest([3,2,1,5,6,4], 2))
print(s.findKthLargest([3,2,3,1,2,4,5,5,6], 4))