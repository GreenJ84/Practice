# You own a Goal Parser that can interpret a string command. The command consists of an alphabet of "G", "()" and/or "(al)" in some order. The Goal Parser will interpret "G" as the string "G", "()" as the string "o", and "(al)" as the string "al". The interpreted strings are then concatenated in the original order.
# Given the string command, return the Goal Parser's interpretation of command.

class Solution:
    def interpret(self, command: str) -> str:
        res = ''
        curr = ''
        for i in command:
            curr+=i
            if curr == 'G':
                res+='G'
                curr = ''
            elif curr == '()':
                res+='o'
                curr = ''
            elif curr == '(al)':
                res+='al'
                curr = ''
        return res

s = Solution()
print(s.interpret("G()(al)"))
print(s.interpret("G()()()()(al)"))
print(s.interpret("(al)G(al)()()G"))
