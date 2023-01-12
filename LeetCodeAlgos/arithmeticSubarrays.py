# A sequence of numbers is called arithmetic if it consists of at least two elements, and the difference between every two consecutive elements is the same. More formally, a sequence s is arithmetic if and only if s[i+1] - s[i] == s[1] - s[0] for all valid i.
# For example, these are arithmetic sequences:
    #* 1, 3, 5, 7, 9
    #* 7, 7, 7, 7
    #* 3, -1, -5, -9
# The following sequence is not arithmetic:
    #* 1, 1, 2, 5, 7
# You are given an array of n integers, nums, and two arrays of m integers each, l and r, representing the m range queries, where the ith query is the range [l[i], r[i]]. All the arrays are 0-indexed.
# Return a list of boolean elements answer, where answer[i] is true if the subarray nums[l[i]], nums[l[i]+1], ... , nums[r[i]] can be rearranged to form an arithmetic sequence, and false otherwise.

from typing import List
class Solution:
    def checkArithmeticSubarrays(self, nums: List[int], l: List[int], r: List[int]) -> List[bool]:
        ans = [True for _ in range(len(l))]
        for i in range(len(l)):
            lvl = sorted(nums[l[i]:r[i]+1])
            d = lvl[0]-lvl[1]
            for j in range(1, len(lvl)-1):
                if lvl[j]-lvl[j+1]!=d:
                    ans[i]=False
                    break
        return ans

# def sequence(arr: List[int]) -> bool:
#     arr.sort()
#     d = arr[0]-arr[1]
#     for i in range(1, len(arr)-1):
#         if arr[i]-arr[i+1]!=d:
#             return False
#     return True

# class Solution:
#     def checkArithmeticSubarrays(self, nums: List[int], l: List[int], r: List[int]) -> List[bool]:
#         ans = [False for _ in range(len(l))]
#         for i in range(len(l)):
#             lvl = [nums[j] for j in range(l[i], r[i]+1)]
#             ans[i]=sequence(lvl)
#         return ans

s = Solution()
print(s.checkArithmeticSubarrays([4,6,5,9,3,7],[0,0,2],[2,3,5]))
print(s.checkArithmeticSubarrays([-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10],[0,1,6,4,8,7],[4,4,9,7,9,10]))