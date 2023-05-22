# Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

# Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

# Return k after placing the final result in the first k slots of nums.

# Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

from typing import List


class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        write_idx = 1
        curr = [1, nums[0]]
        for i in range(1, len(nums)):
            if nums[i] == curr[1]:
                if curr[0] >= 2:
                    continue
                nums[write_idx] = nums[i]
                write_idx += 1
                curr[0] += 1
            else:
                curr[0] = 1
                curr[1] = nums[i]
                nums[write_idx] = curr[1]
                write_idx += 1

        return write_idx

s = Solution()
nums = [1,1,1,2,2,3]
ans = s.removeDuplicates(nums)
print(ans)
assert ans == 5
assert nums == [1,1,2,2,3,3]

nums = [0,0,1,1,1,1,2,3,3]
ans = s.removeDuplicates(nums)
print(ans)
assert ans == 7
assert nums == [0,0,1,1,2,3,3,3,3]
