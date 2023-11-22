# On a 2D plane, we place n stones at some integer coordinate points. Each coordinate point may have at most one stone.
# A stone can be removed if it shares either the same row or the same column as another stone that has not been removed.
# Given an array stones of length n where stones[i] = [xi, yi] represents the location of the ith stone, return the largest possible number of stones that can be removed.

from typing import List


class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        numStones = len(stones)
        clusters = 0
        seen = set()
        def dps(node):
            for stone, location in enumerate(stones):
                if stone not in seen and (location[0] == node[0] or location[1] == node[1]):
                    seen.add(stone)
                    dps(stones[stone])


        for i in range(numStones):
            if i not in seen:
                seen.add(i)
                clusters += 1
                dps(stones[i])
        return numStones - clusters

s = Solution()
print(s.removeStones([[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]))
print(s.removeStones([[0,0],[0,2],[1,1],[2,0],[2,2]]))
print(s.removeStones([[0,0]]))