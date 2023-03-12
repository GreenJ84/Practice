# You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
# Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

from typing import List

class Solution:
    def rob(self, nums: List[int]) -> int:
        res = [0]*(len(nums)+1)
        res[1] = nums[0]
        for i in range(1, len(nums)):
            res[i+1] = max(res[i-1]+nums[i], res[i])
        return res[len(nums)]

# class Solution:
#     def rob(self, nums: List[int]) -> int:
#         match len(nums):
#             case 1:
#                 return nums[0]
#             case 2:
#                 return max(nums[0], nums[1])
#             case 3:
#                 return max(nums[0]+nums[2], nums[1])
#             case _: 
#                 res = [0]*(len(nums)+1)
#                 res[1] = nums[0]
#                 for i in range(1, len(nums)):
#                     res[i+1] = max(res[i-1]+nums[i], res[i])
#                 return res[len(nums)]

s = Solution()
print(s.rob([1,2,3,1]))
print(s.rob([2,7,9,3,1]))