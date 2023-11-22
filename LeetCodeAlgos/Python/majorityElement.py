# Given an array nums of size n, return the majority element.
# The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

from typing import List


class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        count = {}
        for num in nums:
            if num not in count:
                count[num] = 1
            else:
                count[num]+=1
        return max(count.keys(), key=count.get)
    
s = Solution()
print(s.majorityElement([3,2,3]))
print(s.majorityElement([2,2,1,1,1,2,2]))
