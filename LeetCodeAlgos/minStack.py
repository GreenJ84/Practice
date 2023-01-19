# Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
# Implement the MinStack class:
    # MinStack() initializes the stack object.
    # void push(int val) pushes the element val onto the stack.
    # void pop() removes the element on the top of the stack.
    # int top() gets the top element of the stack.
    # int getMin() retrieves the minimum element in the stack.
# You must implement a solution with O(1) time complexity for each function.

from typing import List


class MinStack:

    def __init__(self):
        self.stack = []

    def push(self, val: int) -> None:
        self.stack.append(val)

    def pop(self) -> None:
        self.stack.pop()

    def top(self) -> int:
        print(self.stack[-1])
        return self.stack[-1]

    def getMin(self) -> int:
        print(min(self.stack))
        return min(self.stack)

obj = MinStack()
obj.push(-2)
obj.push(0)
obj.push(-3)
obj.getMin()
obj.pop()
obj.top()
obj.getMin()