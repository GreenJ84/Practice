# You are given a string s that contains some bracket pairs, with each pair containing a non-empty key.
    # For example, in the string "(name)is(age)yearsold", there are two bracket pairs that contain the keys "name" and "age".
# You know the values of a wide range of keys. This is represented by a 2D string array knowledge where each knowledge[i] = [keyi, valuei] indicates that key keyi has a value of valuei.
# You are tasked to evaluate all of the bracket pairs. When you evaluate a bracket pair that contains some key keyi, you will:
    # Replace keyi and the bracket pair with the key's corresponding valuei.
    # If you do not know the value of the key, you will replace keyi and the bracket pair with a question mark "?" (without the quotation marks).
# Each key will appear at most once in your knowledge. There will not be any nested brackets in s.

# Return the resulting string after evaluating all of the bracket pairs.

from typing import List
from collections import defaultdict


class Solution:
    def evaluate(self, s: str, knowledge: List[List[str]]) -> str:
        map = defaultdict(str)
        for k, v in knowledge:
            map[k] = v
        start = 0
        end = 0
        curr = 0
        while curr < len(s):
            if s[curr] == '(':
                start = curr
            elif s[curr] == ')':
                end = curr
                key = s[start + 1 : end]
                value = None
                if key in map:
                    value = map[key]
                if value is None:
                    value = '?'
                s = s[:start] + value + s[end + 1:]
                curr = start + len(value) -1
            curr += 1
        return s

s = Solution()
test1 = s.evaluate("(name)is(age)yearsold", [["name", "bob"], ["age", "two"]])
print(test1)
assert test1 == "bobistwoyearsold"

test2 = s.evaluate("hi(name)", [["a", "b"]])
print(test2)
assert test2 == "hi?"

test3 = s.evaluate("(a)(a)(a)aaa", [["a","yes"]])
print(test3)
assert test3 == "yesyesyesaaa"