# Given a string s, sort it in decreasing order based on the frequency of the characters. The frequency of a character is the number of times it appears in the string.
# Return the sorted string. If there are multiple answers, return any of them.

class Solution:
    def frequencySort(self, s: str) -> str:
        cnt = {}
        for i in s:
            if i in cnt:
                cnt[i] += 1
            else:
                cnt[i] = 1
        res = sorted(cnt.items(), key=lambda x: x[1], reverse=True)
        resStr = ""
        for i in res:
            for j in range(i[1]):
                resStr += i[0]
        return resStr
    
s = Solution()
print(s.frequencySort("tree"))
print(s.frequencySort("cccaaa"))
print(s.frequencySort("Aabb"))