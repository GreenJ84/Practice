// Given the root of a perfect binary tree, reverse the node values at each odd level of the tree.

// For example, suppose the node values at level 3 are [2,1,3,4,7,11,29,18], then it should become [18,29,11,7,4,3,1,2].
// Return the root of the reversed tree.

// A binary tree is perfect if all parent nodes have two children and all leaves are on the same level.

// The level of a node is the number of edges along the path between it and the root node.
import java.util.*;

public class ReverseOddLevelsOfBinaryTree {
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
  public static void main(String[] args) {}

  public TreeNode reverseOddLevels(TreeNode root) {
    if (root.left == null) return root;
    List<TreeNode> curr = new ArrayList<>(), nxt = new ArrayList<>();
    curr.add(root);
    int lvlNum = 0;

    while (!curr.isEmpty()){
      for (int i = 0; i < curr.size(); i++){
        TreeNode currNode = curr.get(i);
        if (i == 0 && currNode.left == null) break;
        nxt.add(currNode.left);
        nxt.add(currNode.right);
      }
      curr = nxt;
      nxt = new ArrayList<>();
      ++lvlNum;
      if (lvlNum % 2 == 1) {
        int n = curr.size();
        for (int i = 0; i < n / 2; i++){
          int temp = curr.get(i).val;
          curr.get(i).val = curr.get(n - i - 1).val;
          curr.get(n - i - 1).val = temp;
        }
      }
    }
    return root;
  }
}
