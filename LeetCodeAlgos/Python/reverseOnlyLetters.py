# Given a string s, reverse the string according to the following rules:

# All the characters that are not English letters remain in the same position.
# All the English letters (lowercase or uppercase) should be reversed.
# Return s after reversing it.

class Solution:
    def reverseOnlyLetters(self, s: str) -> str:
        char = list( filter(lambda x: x.isalpha(), s))
        ans = []
        for ch in s:
            if ch.isalpha():
                ans.append( char.pop() )
            else:
                ans.append( ch )
        return "".join(ans)
    
s = Solution()
print(s.reverseOnlyLetters("ab-cd"))
print(s.reverseOnlyLetters("a-bC-dEf-ghIj"))
print(s.reverseOnlyLetters("Test1ng-Leet=code-Q!"))