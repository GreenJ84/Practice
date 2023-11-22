# You are given a nested list of integers nestedList. Each element is either an integer or a list whose elements may also be integers or other lists. Implement an iterator to flatten it.
# Implement the NestedIterator class:
    # NestedIterator(List<NestedInteger> nestedList) Initializes the iterator with the nested list nestedList.
    # int next() Returns the next integer in the nested list.
    # boolean hasNext() Returns true if there are still some integers in the nested list and false otherwise.

class NestedInteger:
    def __init__(self) -> None:
        pass

    def isInteger(self) -> bool:
        pass

    def getInteger(self) -> int:
        pass

    def getList(self):
        pass

class NestedIterator:
    def __init__(self, nestedList):
        def addThrough(list, nested):
            while nested:
                if nested[-1].isInteger():
                    list.append(nested[-1].getInteger())
                    nested.pop()
                else:
                    list = addThrough(list, nested.getList()[::-1])
                    nested.pop()
            return list
        list = []
        nestedList = nestedList[::-1]
        list = addThrough(list, nestedList)
        self.list = list
        self.idx = 0

    def next(self) -> int:
        self.idx+=1
        return self.list[self.idx-1]
    
    def hasNext(self) -> bool:
        return True if self.idx < len(self.list) else False

## Your NestedIterator object will be instantiated and called as such:
i, v = NestedIterator([1,[4,[6]]]), []
while i.hasNext(): v.append(i.next())
print(v)