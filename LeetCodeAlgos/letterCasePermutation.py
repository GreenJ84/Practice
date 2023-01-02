# Given a string s, you can transform every letter individually to be lowercase or uppercase to create another string.
# Return a list of all possible strings we could create. Return the output in any order.

from typing import List


class Solution:
    def letterCasePermutation(self, s: str) -> List[str]:

        # String builder function
        def backtrack(str, index):
            # Append all finished strings
            if len(str) == len(s):
                ans.append(''.join(str))
                return 

            # Itterate through the langth of what our strings should be
            for i in range(index, len(s)):

                # Just add digits to strings and procede
                if s[i].isdigit():
                    backtrack(str+s[i], i+1)

                # For characters, add both the lower and upper versions before preceeding
                if s[i].isalpha():
                    # Lower
                    backtrack(str+s[i].lower(), i+1)
                    # Upper
                    backtrack(str+s[i].upper(), i+1)

        # Create an answer for resturn string array
        ans = []
        # Start building string sout of s
        backtrack('', 0)
        return ans

# class Solution:
#     def letterCasePermutation(self, s: str) -> List[str]:
#         if s.isnumeric(): return s
#         res = [s.upper(), s.lower()]

#         s = list(s.lower())
#         idx = []
#         for i in range(len(s)):
#             if not s[i].isnumeric():
#                 idx.append(i)
#                 s[i] = toggle(s[i])
#                 if ''.join(s) not in res:
#                     res.append(''.join(s))
#         while len(res)!=len(idx)**2:
#             for i in idx:
#                 s[i] = toggle(s[i])
#                 if ''.join(s) not in res:
#                     res.append(''.join(s))
#         return res

# def toggle(i: str) -> str:
#     if i == i.upper():
#         return i.lower()
#     else:
#         return i.upper()

s = Solution()
# print(s.letterCasePermutation('a1b2'))
# print(s.letterCasePermutation('3z4'))
print(s.letterCasePermutation('Jrq'))