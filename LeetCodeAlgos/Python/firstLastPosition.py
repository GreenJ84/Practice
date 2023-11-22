# Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
# If target is not found in the array, return [-1, -1].
#! You must write an algorithm with O(log n) runtime complexity.

from typing import List

class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        res = [-1, -1]
        if target in nums:
            def search(start, end):
                m = start+((end-start)//2)
                if nums[start] == target:
                    res[0] = start
                    e = start+1
                    while e < len(nums) and nums[e] == target:
                        e+=1
                    res[1] = e-1
                    return
                if nums[end] == target and res[1] == -1:
                    res[1] = end
                    s = end-1
                    while s > -1 and nums[s] == target:
                        s-=1
                    res[0] = s+1
                    return
                if nums[m] == target:
                    if res[0] == -1:
                        s = m-1
                        while s > -1 and nums[s] == target:
                            s-=1
                        res[0] = s+1
                    if res[1] == -1:
                        e = m+1
                        while e < len(nums) and nums[e] == target:
                            e +=1
                        res[1] = e-1
                    return
                elif nums[m] < target:
                    search(m+1, end)
                else:
                    search(start, m-1)
            search(0, len(nums)-1)
        return res

# class Solution:
#     def searchRange(self, nums: List[int], target: int) -> List[int]:
#         if target not in nums:
#             return [-1,-1]
#         else:
#             res = []
#             res.append(nums.index(target))
#             for i in range(res[0]+1, len(nums)):
#                 if nums[i] != target:
#                     res.append(i-1)
#                     break
#             if i == len(nums):
#                 res.append(len(nums)-1)
#         return res

s = Solution()
print(s.searchRange([5,7,7,8,8,10],8))
print(s.searchRange([5,7,7,8,8,10],6))
print(s.searchRange([],0))
print(s.searchRange([0,0,1,1,1,2,4,4,4,4,5,5,5,6,8,8,9,9,10,10,10], 8))