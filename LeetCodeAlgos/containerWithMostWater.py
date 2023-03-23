# You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
# Find two lines that together with the x-axis form a container, such that the container contains the most water.
# Return the maximum amount of water a container can store.
# Notice that you may not slant the container.

from typing import List


class Solution:
    def maxArea(self, height: List[int]) -> int:
        n = len(height)
        ans = 0
        for i in range(n):
            lvlMax = 0
            for j in range(i+1, n):
                lvlMax = max(lvlMax, (j-i)*min(height[i], height[j]))
            ans = max(ans, lvlMax)
        return ans

s = Solution()
print(s.maxArea([1,8,6,2,5,4,8,3,7]))
print(s.maxArea([1,1]))
