class Solution:
    def removeDuplicates(self, s: str) -> str:
        s, x, i = list(s), len(s), 0
        while(i < x-1):
            if s[i] == s[i+1]:
                s.pop(i+1)
                s.pop(i)
                x-=2
                if i == 0:
                    i-=1
                else:
                    i-=2
            i+=1
        return ''.join(s)

s = Solution()
print(s.removeDuplicates("abbaca"))
print(s.removeDuplicates("azxxzy"))
print(s.removeDuplicates("aaaaaaaa"))
print(s.removeDuplicates("aaaaaaaaa"))
# print(s.removeDuplicates("Where in the heckk dooes thee dupliccatess coe frrrooommmmmmmmmm"))