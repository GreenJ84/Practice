// Given a binary tree root and an integer target, delete all the leaf nodes with value target.

// Note that once you delete a leaf node with value target, if its parent node becomes a leaf node and has the value target, it should also be deleted (you need to continue doing that until you cannot).
public class DeleteLeavesWithAgivenValue {
  
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
    public TreeNode removeLeafNodes(TreeNode root, int target) {
      if (root == null) throw new IllegalArgumentException();
      return checkNode(root, target) ?  null : root;
    }

    public boolean checkNode(TreeNode node, int target){
      if (node.left != null && checkNode(node.left, target)){
          node.left = null;
      }
      if (node.right != null && checkNode(node.right, target)){
        node.right = null;
      }

      if (node.left == null && node.right == null && node.val == target){
        return true;
      }
      return false;
    }
}