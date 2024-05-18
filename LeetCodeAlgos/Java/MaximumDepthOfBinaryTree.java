// Given the root of a binary tree, return its maximum depth.

// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

public class MaximumDepthOfBinaryTree {
  public static void main(String[] args) {
    
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
}

class Solution {
  private Integer depth = 0;
  public int maxDepth(TreeNode root) {
      if (root == null){ return this.depth; }
      this.depth++;
      dps(root.left, 1);
      dps(root.right, 1);
      return this.depth;
  }

  private void dps(TreeNode node, Integer depth) {
    // Base case, return if node is null
    if (node == null){ return; }
    depth++;
    // Check depth on leaves only
    if (node.left == null && node.right == null && depth > this.depth){
      this.depth = depth;
    }
    // Recurse on left and right subtrees
    dps(node.left, depth);
    dps(node.right, depth);
  }
}