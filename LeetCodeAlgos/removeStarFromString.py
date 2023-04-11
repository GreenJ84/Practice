# You are given a string s, which contains stars *.
# In one operation, you can:
    # Choose a star in s.
    # Remove the closest non-star character to its left, as well as remove the star itself.
# Return the string after all stars have been removed.
# Note:
    # The input will be generated such that the operation is always possible.
    # It can be shown that the resulting string will always be unique.
class Solution:
    def removeStars(self, s: str) -> str:
        ans = []
        for char in s:
            if char == '*':
                if ans:
                    ans.pop()
                continue
            ans.append(char)
        return ''.join(ans)

# class Solution:
#     def removeStars(self, s: str) -> str:
#         n = len(s)
#         if s.count('*') > n/2: 
#             return ''

#         while s[0] == '*':
#             s = s[1:]
#             n -= 1

#         char = 0
#         while char < n-1:
#             if s[char+1] == '*':
#                 s = s[:char]+s[char+2:]
#                 char -= 1
#                 n -= 2
#             else:
#                 char += 1
#         return s

s = Solution()
print(s.removeStars("leet**cod*e"))
print(s.removeStars("erase*****"))
print(s.removeStars("*****"))
print(s.removeStars("*****a"))
print(s.removeStars("a*a*a*a*a*leffover"))
print(s.removeStars("nice*a****"))