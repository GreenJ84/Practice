# Write a function to find the longest common prefix string amongst an array of strings.
# If there is no common prefix, return an empty string "".



from typing import List


class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        res = strs[0]
        for j in range(1, len(strs)):
            if len(res)>len(strs[j]):
                res = res [:len(strs[j])]
            for i in range(0, len(res)):
                if res[i] != strs[j][i]:
                    res = res[:i]
                    break
        ''.join(res)
        return res

s = Solution()
print(s.longestCommonPrefix(["flower","flow","flight"]))
print(s.longestCommonPrefix(["dog","racecar","car"]))
print(s.longestCommonPrefix(['ab', 'a']))