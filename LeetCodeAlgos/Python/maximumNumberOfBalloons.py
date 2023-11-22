# Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.

# You can use each character in text at most once. Return the maximum number of instances that can be formed.

from collections import defaultdict


class Solution:
    def maxNumberOfBalloons(self, text: str) -> int:
        if len(text) < 7 or len(set(text)) < 5:
            return 0
        
        track = defaultdict(int)
        for c in text:
            if c in track:
                track[c] += 1
            else:
                track[c] = 1

        return min([track['b'], track['a'], track['l'] // 2, track['o'] // 2, track['n']])
    
s = Solution()
print(s.maxNumberOfBalloons("loonbalxballpoon"))
print(s.maxNumberOfBalloons("leetcode"))