# Given a triangle array, return the minimum path sum from top to bottom.
# For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

from typing import List


class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        if len(triangle) == 1:
            return triangle[0][0]
        lvl, pre = triangle[len(triangle)-2], triangle[len(triangle)-1]
        x=3

        while len(pre)>1:
            for i in range(len(lvl)):
                lvl[i]+=min(pre[i], pre[i+1])
            lvl, pre = triangle[len(triangle)-x], lvl
            x+=1
        return pre[0]

s = Solution()
print(s.minimumTotal([[2],[3,4],[6,5,7],[4,1,8,3]]))
print(s.minimumTotal([[-10]]))