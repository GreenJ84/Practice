// Given the root of a binary tree and two integers val and depth, add a row of nodes with value val at the given depth depth.
// Note that the root node is at depth 1.
// The adding rule is:
  // Given the integer depth, for each not null tree node cur at the depth depth - 1, create two tree nodes with value val as cur's left subtree root and right subtree root.
  // cur's original left subtree should be the left subtree of the new left subtree root.
  // cur's original right subtree should be the right subtree of the new right subtree root.
// If depth == 1 that means there is no depth depth - 1 at all, then create a tree node with value val as the new root of the whole original tree, and the original tree is the new root's left subtree.

import java.util.ArrayList;
import java.util.List;

public class AddOneRowToTree {
  
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
  public TreeNode addOneRow(TreeNode root, int val, int depth) {
      if (depth == 1){
          return new TreeNode(val, root, null);
      }
      List<TreeNode> lvl = new ArrayList<>();
      lvl.add(root);
      List<TreeNode> nxt = new ArrayList<>();
      for (int i = 1; i < depth - 1; i++){
        for (TreeNode node : lvl){
          if (node.left!= null){
            nxt.add(node.left);
          }
          if (node.right!= null){
            nxt.add(node.right);
          }
        }
        lvl = nxt;
        nxt = new ArrayList<>();
      }
      for (TreeNode node : lvl){
        TreeNode temp = node.left;
        node.left = new TreeNode(val, temp, null);
        temp = node.right;
        node.right = new TreeNode(val, null, temp);
      }
      return root;
  }
}