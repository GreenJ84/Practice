# Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a backspace character.
# Note that after backspacing an empty text, the text will continue empty.

class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        s, i = list(s), 0
        while(i < len(s)):
            if s[i] == '#':
                if i == 0:
                    s.pop(i)
                    i-=1
                else:
                    s.pop(i), s.pop(i-1)
                    i-=2
            i+=1
        t, i = list(t), 0
        while(i < len(t)):
            if t[i] == '#':
                if i == 0:
                    t.pop(i)
                    i-=1
                else:
                    t.pop(i), t.pop(i-1)
                    i-=2
            i+=1

        return True if t == s else False

s = Solution()
print(s.backspaceCompare('ab#c', 'ad#c'))
print(s.backspaceCompare('ab##', 'c#d#'))
print(s.backspaceCompare('a#c', 'b'))