# You are given an array of integers stones where stones[i] is the weight of the ith stone.
# We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together. Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:
# If x == y, both stones are destroyed, and
# If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
# At the end of the game, there is at most one stone left.
# Return the weight of the last remaining stone. If there are no stones left, return 0.

from typing import List


class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        if len(stones) == 1:
            return stones[0]
        stones.sort()
        x = len(stones)-1
        while len(stones)>1:
            if stones[x] != stones[x-1]:
                stones[x] = stones[x]-stones[x-1]
                stones.pop(x-1)
                x = len(stones)-1
                stones.sort()
            else:
                stones.pop(x)
                stones.pop(x-1)
                x-=2
        return max(stones[0], 0)

s = Solution()
print(s.lastStoneWeight([2,7,4,1,8,1]))
print(s.lastStoneWeight([1]))