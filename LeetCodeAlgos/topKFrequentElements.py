# Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

from typing import List


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        cnt = {}
        for i in nums:
            if i in cnt:
                cnt[i] += 1
            else: 
                cnt[i] = 1
        res = sorted(cnt.items(), key = lambda x: x[1], reverse=True)[:k]
        return [i[0] for i in res]

s = Solution()
print(s.topKFrequent([1,1,1,2,2,3], 2))
print(s.topKFrequent([1], 1))
print(s.topKFrequent([4,1,-1,2,-1,2,3], 2))