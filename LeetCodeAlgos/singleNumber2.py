# Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
# You must implement a solution with a linear runtime complexity and use only constant extra space.



from typing import List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return nums[0]
        nums.sort()
        if nums[0]!= nums[1]:
            return nums[0]
        for i in range(1, len(nums) - 1):
            if nums[i] != nums[i -1] and nums[i] != nums[i+1]:
                return nums[i]
        return nums[-1]


s = Solution()
print(s.singleNumber([2,2,3,2]))
print(s.singleNumber([0,1,0,1,0,1,99]))