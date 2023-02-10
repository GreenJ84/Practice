# Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
# Notice that the solution set must not contain duplicate triplets.

from typing import List

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        res = []
        if len(nums) < 3: return res
        nums.sort()
        target = 0

        for i in range(len(nums)-2):
            if nums[i] > target: break
            if i>0 and nums[i] == nums[i-1]: continue
            j, k = i+1, len(nums)-1

            while j < k:
                sum = nums[i]+nums[j]+nums[k]
                if sum == target:
                    res.append([nums[i], nums[j], nums[k]])
                    while(j<len(nums)-1 and nums[j] == nums[j+1]): j+=1
                    while(k > 0 and nums[k] == nums[k-1]): k-=1
                    j+=1
                    k-=1
                elif sum > target:
                    k-=1
                else:
                    j+=1
        return res


s = Solution()
# print(s.threeSum([-1,0,1,2,-1,-4]))
# print(s.threeSum([0,1,1]))
print(s.threeSum([0,0,0]))