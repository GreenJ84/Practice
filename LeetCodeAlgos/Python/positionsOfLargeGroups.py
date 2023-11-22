# In a string s of lowercase letters, these letters form consecutive groups of the same character.

# For example, a string like s = "abbxxxxzyy" has the groups "a", "bb", "xxxx", "z", and "yy".

# A group is identified by an interval [start, end], where start and end denote the start and end indices (inclusive) of the group. In the above example, "xxxx" has the interval [3,6].

# A group is considered large if it has 3 or more characters.

# Return the intervals of every large group sorted in increasing order by start index.

from typing import List


class Solution:
    def largeGroupPositions(self, s: str) -> List[List[int]]:
        ans = []
        run = 1
        for i in range(1, len(s)):
            if s[i] == s[i - 1]:
                run += 1
            else:
                if run >= 3:
                    ans.append([i - run, i - 1])
                run = 1

        if run >= 3:
            ans.append([i - run + 1, i])
        return ans
    
s = Solution()
print(s.largeGroupPositions("abbxxxxzyy"))
print(s.largeGroupPositions("abc"))
print(s.largeGroupPositions("abcdddeeeeaabbbcd"))
print(s.largeGroupPositions("aaa"))