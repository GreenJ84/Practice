// Given the root of a binary tree, invert the tree, and return its root.

import java.util.*;

public class InvertBinaryTree {
  public static void main(String[] args) {
    Solution s = new Solution();

    TreeNode tree1 = new TreeNode(4,
      new TreeNode(2,
        new TreeNode(1),
        new TreeNode(3)
      ),
      new TreeNode(7,
        new TreeNode(6),
        new TreeNode(9)
      )
    );
    TreeNode expected1 = new TreeNode(4,
      new TreeNode(7,
        new TreeNode(6),
        new TreeNode(9)
      ),
      new TreeNode(2,
        new TreeNode(1),
        new TreeNode(3)
      )
    );
    testInvertTree(1, tree1, expected1, s);

    TreeNode tree2 = new TreeNode(2,
      new TreeNode(1),
      new TreeNode(3)
    );
    TreeNode expected2 = new TreeNode(2,
      new TreeNode(3),
      new TreeNode(1)
    );
    testInvertTree(2, tree2, expected2, s);
  }

  static void testInvertTree(int testNum, TreeNode root, TreeNode expected, Solution s){
    TreeNode result = s.invertTree(root);

    System.out.println(String.format(
      "Test %d: %b",
      testNum,
      result.equals(expected)
    ));
  }
}

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode() {}
    TreeNode(int val) { this.val = val; }
    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }

    public boolean equals(TreeNode other){
      return this.val == other.val && this.left.equals(other.left) && this.right.equals(other.right);
    }
}

class Solution {
  public TreeNode invertTree(TreeNode root) {
    if (root == null) return root;

    Queue<TreeNode> nxt = new LinkedList<TreeNode>();
    nxt.add(root);

    while (!nxt.isEmpty()){
      TreeNode node = nxt.poll();
      if (node.left == null && node.right == null){
        continue;
      }
      if (node.left != null){
        nxt.add(node.left);
      }
      if (node.right!= null){
        nxt.add(node.right);
      }
      TreeNode temp = node.left;
      node.left = node.right;
      node.right = temp;
    }
    return root;
  }
}