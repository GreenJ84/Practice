# You are given an integer array nums consisting of n elements, and an integer k.

# Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.

from typing import List


class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        mx = curr = sum(nums[:k]) / k
        for i in range(k, len(nums)):
            curr -= nums[i - k] / k
            curr += nums[i] / k
            mx = max(mx, curr)
        return mx