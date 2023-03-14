# Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
# Return the sum of the three integers.
# You may assume that each input would have exactly one solution.

from typing import List


class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        if len(nums) == 3:
            return sum(nums)
        n = len(nums)
        nums.sort()
        ans = float('inf')

        for i in range(n-2):
            if i > 0 and nums[i] == nums[i-1]: 
                continue
            j, k = i+1, n-1

            while j < k:
                # Check total and answer distances
                total = nums[i]+nums[j]+nums[k]
                if total == target:
                    return total
                elif abs(target-total) < abs(target-ans):
                    ans = total
                # Move the pointers
                if total < target:
                    j+=1
                elif total > target:
                    k-=1
        return ans

s = Solution()
print(s.threeSumClosest([-1,2,1,-4], 1))
print(s.threeSumClosest([0,0,0], 0))
print(s.threeSumClosest([2,1,4,6,8,0], 0))