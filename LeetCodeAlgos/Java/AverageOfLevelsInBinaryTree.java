// Given the root of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within 10-5 of the actual answer will be accepted.

import java.util.*;

public class AverageOfLevelsInBinaryTree {
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
  public List<Double> averageOfLevels(TreeNode root) {
    List<TreeNode> lvl = new ArrayList<TreeNode>();
    List<TreeNode> next = new ArrayList<TreeNode>();
    List<Double> res = new ArrayList<Double>();

    lvl.add(root);
    while (lvl.size() > 0){
      double sum = 0;
      for (TreeNode node: lvl){
        sum += node.val;
        if (node.left!= null){
          next.add(node.left);
        }
        if (node.right!= null){
          next.add(node.right);
        }
      }
      res.add(sum / lvl.size());
      lvl = next;
      next = new ArrayList<TreeNode>();
    }
    return res;
  }
}