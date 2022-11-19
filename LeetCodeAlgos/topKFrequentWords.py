# Given an array of strings words and an integer k, return the k most frequent strings.
# Return the answer sorted by the frequency from highest to lowest. Sort the words with the same frequency by their lexicographical order.

from typing import List


class Solution:
    def topKFrequent(self, words: List[str], k: int) -> List[str]:
        words.sort()
        count = {}
        for word in words:
            if word not in count:
                count[word] = 1
            else:
                count[word]+=1
        count = sorted(count, key = lambda i: count[i], reverse=True)
        while(len(count) != k):
            count.pop(k)
        return count




s = Solution()
print(s.topKFrequent(["i","love","leetcode","i","love","coding"], 2))
print(s.topKFrequent(["the","day","is","sunny","the","the","the","sunny","is","is"], 4))

