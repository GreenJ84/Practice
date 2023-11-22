# Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

class Solution:
    def reverseWords(self, s: str) -> str:
        x = s.split()
        for n in range(len(x)):
            y = x[n]
            x[n] = reverse(list(y))
        x = " ".join(x)
        return x

def reverse(n):
    for i in range(len(n)//2):
        n[i], n[-(i+1)] = n[-(i+1)], n[i]
    return "".join(n)

s = Solution()
print(s.reverseWords("Let's take LeetCode contest"))
print(s.reverseWords("God Ding"))