# Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
# Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
# Clarification: The input/output format is the same as how LeetCode serializes a binary tree. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.

class TreeNode(object):
    def __init__(self, x, left = None, right=None):
        self.val = x
        self.left = left
        self.right = right
    def __eq__(self, __o: object) -> bool:
        return self.__dict__ == __o.__dict__

class Codec:
    def serialize(self, root):
        """Encodes a tree to a single string."""
        ser = []
        def preorder(node):
            if node: 
                ser.append(str(node.val))
            else: return None
            preorder(node.left) if node.left else ser.append("")
            preorder(node.right) if node.right else ser.append("")
        preorder(root)
        return ",".join(ser)

    def deserialize(self, data):
        """Decodes your encoded data to tree."""
        def decode(q: list):
            curr = q.pop(0)
            if not curr: return None
            node = TreeNode(int(curr))
            node.left = decode(q)
            node.right = decode(q)
            return node

        queue: list = list(data.split(","))
        return decode(list(queue))

tree = TreeNode(1, TreeNode(2), TreeNode(3, TreeNode(4), TreeNode(5)))
ser = Codec()
deser = Codec()
ans = deser.deserialize(ser.serialize(
        tree
    ))
print(ans == tree)
ans = deser.deserialize(ser.serialize(
        None
    ))
print(ans == None)