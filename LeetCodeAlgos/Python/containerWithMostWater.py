# You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
# Find two lines that together with the x-axis form a container, such that the container contains the most water.
# Return the maximum amount of water a container can store.
# Notice that you may not slant the container.

from typing import List

class Solution:
    def maxArea(self, height: List[int]) -> int:
        ans = 0
        start, end = 0, len(height)-1

        while start < end:
            ans = max(ans, min(height[start], height[end])*(end-start))
            if height[start] < height[end]:
                start += 1
                while height[start] < height[start-1]:
                    start += 1
            else:
                end -= 1
                while height[end] < height[end+1]:
                    end -= 1
        return ans

# class Solution:
#     def maxArea(self, height: List[int]) -> int:
#         n = len(height)
#         ans = 0
#         for i in range(n):
#             lvlMax = 0
#             for j in range(i+1, n):
#                 lvlMax = max(lvlMax, (j-i)*min(height[i], height[j]))
#             ans = max(ans, lvlMax)
#         return ans

s = Solution()
print(s.maxArea([1,8,6,2,5,4,8,3,7]))
print(s.maxArea([1,1]))
