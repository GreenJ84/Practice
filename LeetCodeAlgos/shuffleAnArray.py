# Given an integer array nums, design an algorithm to randomly shuffle the array. All permutations of the array should be equally likely as a result of the shuffling.
# Implement the Solution class:
    # Solution(int[] nums) Initializes the object with the integer array nums.
    # int[] reset() Resets the array to its original configuration and returns it.
    # int[] shuffle() Returns a random shuffling of the array.

from typing import List
import random


class Solution:

    def __init__(self, nums: List[int]):
        self.truNums = nums
        self.nums = nums[:]
        self.length = len(nums)
        

    def reset(self) -> List[int]:
        self.nums = self.truNums[:]
        print(self.nums)
        return self.nums

    def shuffle(self) -> List[int]:
        for _ in range(self.length):
            to = random.randint(0, self.length*100) % self.length - 1
            fro = random.randint(0, self.length*100) % self.length - 1
            self.nums[to], self.nums[fro] = self.nums[fro], self.nums[to]
        print(self.nums)
        return self.nums
    
s1 = Solution([1, 2, 3])
s1.shuffle()
s1.reset()
s1.shuffle()

s2 = Solution([4, 8, 12, 16])
s2.shuffle()
s2.reset()
s2.shuffle()
