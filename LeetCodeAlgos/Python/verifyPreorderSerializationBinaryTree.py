# One way to serialize a binary tree is to use preorder traversal. When we encounter a non-null node, we record the node's value. If it is a null node, we record using a sentinel value such as '#'.

class Solution:
    def isValidSerialization(self, preorder: str) -> bool:
        if len(preorder) < 3:
            if len(preorder) == 1 and preorder[0] == '#':
                return True
            return False
        
        stack = []
        for s in preorder.split(","):
            stack.append(s)
            while len(stack) > 2 and stack[-1] == "#" and stack[-2] == "#" and stack[-3] != '#':
                for _ in range(3):
                    stack.pop()
                stack.append('#')
        return len(stack) == 1 and stack[0] == '#'

s = Solution()
print( s.isValidSerialization("9,3,4,#,#,1,#,#,2,#,6,#,#"))
print( s.isValidSerialization( "1,#" ) )
print( s.isValidSerialization( "9,#,#,1" ) )