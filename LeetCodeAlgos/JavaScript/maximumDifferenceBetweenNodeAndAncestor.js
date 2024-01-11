// Given the root of a binary tree, find the maximum value v for which there exist different nodes a and b where v = |a.val - b.val| and a is an ancestor of b.
// A node a is an ancestor of b if either: any child of a is equal to b or any child of a is an ancestor of b.


class TreeNode {
  val;
  left;
  right;

  constructor(val, left, right) {
    this.val = val ?? 0;
    this.left = left ?? null;
    this.right = right ?? null;
  }
}

/**
 * @param {TreeNode} root
 * @return {number}
 */
const maxAncestorDiff = function (root) {
  if (!root || (!root.left && !root.right)) return -1;
  let ans = 0;
  
  const dfs = (node, ancestors) => {
    // Get left child diff
    if (node.left) {
      ans = Math.max(ans, Math.abs(node.val - node.left.val));
      dfs(node.left, [...ancestors, node.val]);
    };
    // Get right child diff
    if (node.right) {
      ans = Math.max(ans, Math.abs(node.val - node.right.val));
      dfs(node.right, [...ancestors, node.val]);
    }
    // Get diffs of all ancestors minus parent node
    for (let i = ancestors.length - 2; i >= 0; i--) { 
      ans = Math.max(ans, Math.abs(ancestors[i] - node.val));
    }
  }
  dfs(root, []);
  return ans;
};

function testMaxAncestorDiff(testNum, tree, expected) { 
  let result = maxAncestorDiff(tree);
  console.log(
    `Test ${testNum}: `,
    result,
    result === expected? " Pass ✅" : " Fail ❌"
  );
}

const tree1 = new TreeNode(
  8,
  new TreeNode(
    3,
    new TreeNode(
      1
    ),
    new TreeNode(
      6,
      new TreeNode(
        4,
      ),
      new TreeNode(
        7
      )
    )
  ),
  new TreeNode(
    10,
    null,
    new TreeNode(
      14,
      new TreeNode(
        13
      )
    )
  )
);
testMaxAncestorDiff(1, tree1, 7);

const tree2 = new TreeNode(
  1,
  null,
  new TreeNode(
    2,
    null,
    new TreeNode(
      0,
      new TreeNode(
        3
      )
    )
  )
);
testMaxAncestorDiff(2, tree2, 3);