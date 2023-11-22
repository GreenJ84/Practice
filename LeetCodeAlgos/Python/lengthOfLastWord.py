# Given a string s consisting of words and spaces, return the length of the last word in the string.
# A word is a maximal substring consisting of non-space characters only.

#* Improve Solution 
class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        # Strip right side whitespace
        # Split up the string to word sections
        # Return the last words length
        return len(s.rstrip().split()[-1])

## Initial solution - Reduced performance with seperated functions
# class Solution:
#     def lengthOfLastWord(self, s: str) -> int:
#         s = s.strip()
#         s = s.split(' ')
#         return len(s[-1])

s = Solution()
print(s.lengthOfLastWord("Hello World"))
print(s.lengthOfLastWord("   fly me   to   the moon  "))
print(s.lengthOfLastWord("luffy is still joyboy"))