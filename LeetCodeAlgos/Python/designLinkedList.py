# Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
# A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.
# If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.
# Implement the MyLinkedList class:
    # MyLinkedList() Initializes the MyLinkedList object.
    # int get(int index) Get the value of the indexth node in the linked list. If the index is invalid, return -1.
    # void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
    # void addAtTail(int val) Append a node of value val as the last element of the linked list.
    # void addAtIndex(int index, int val) Add a node of value val before the indexth node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
    # void deleteAtIndex(int index) Delete the indexth node in the linked list, if the index is valid.

class Node:
    def __init__(self, val=0, next=None, prev=None, index=None) -> None:
        self.val = val
        self.next = next
        self.prev = prev

class MyLinkedList:

    def __init__(self):
        self.head = None
        self.tail = None

    def get(self, index: int) -> int:
        if not self.head:
            return -1
        runner, idx = self.head, 0
        while runner:
            if idx == index:
                return runner.val
            runner = runner.next
            idx += 1
        return -1

    def addAtHead(self, val: int) -> None:
        if not self.head:
            self.head = self.tail = Node(val)
            return
        temp = self.head
        self.head = Node(val, temp)
        temp.prev = self.head

    def addAtTail(self, val: int) -> None:
        if not self.tail:
            self.head = self.tail = Node(val)
            return
        temp = self.tail
        self.tail = Node(val, None, temp)
        temp.next = self.tail

    def addAtIndex(self, index: int, val: int) -> None:
        if not self.head:
            if index != 0:
                return
            else:
                self.addAtHead(val)
        else:
            if index == 0:
                self.addAtHead(val)
                return
            runner, idx = self.head, 0
            while runner:
                if idx == index-1:
                    if not runner.next:
                        self.addAtTail(val)
                        return
                    temp = runner.next
                    runner.next = Node(val, temp, runner)
                    temp.prev = runner.next
                    return
                runner = runner.next
                idx += 1


    def deleteAtIndex(self, index: int) -> None:
        if not self.head:
            return
        runner, idx = self.head, 0
        while runner:
            if idx == index:
                if runner == self.head:
                    if not runner.next:
                        self.head = self.tail = None
                        return
                    self.head = runner.next
                    runner.next = None
                    self.head.prev = None
                    return
                elif runner == self.tail:
                    self.tail = runner.prev
                    runner.prev = None
                    self.tail.next = None
                    return
                front = runner.prev
                after = runner.next
                front.next, after.prev = after, front
                runner = None
                return
            runner = runner.next
            idx += 1


# Your MyLinkedList object will be instantiated and called as such:
# obj = MyLinkedList()
# obj.addAtHead(1)
# obj.addAtTail(3)
# obj.addAtIndex(1,2)
# print(obj.get(1))
# obj.deleteAtIndex(1)
# print(obj.get(1))


obj2 = MyLinkedList()
obj2.addAtHead(7)
obj2.addAtHead(2)
obj2.addAtHead(1)
obj2.addAtIndex(3,0)
obj2.deleteAtIndex(2)
obj2.addAtHead(6)
obj2.addAtTail(4)
print(obj2.get(4))
obj2.addAtHead(4)
obj2.addAtIndex(5,0)
obj2.addAtHead(6)