/** @format */

// You are given the root of a binary tree with unique values, and an integer start. At minute 0, an infection starts from the node with value start.
// Each minute, a node becomes infected if:
// The node is currently uninfected.
// The node is adjacent to an infected node.
// Return the number of minutes needed for the entire tree to be infected.

class TreeNode{
  constructor(val, left, right) {
    this.val = val ?? 0;
    this.left = left ?? null;
    this.right = right ?? null;
  }
}
/**
 * @param {TreeNode} root
 * @param {number} start
 * @return {number}
 */

/**
 - I want to find the longest routes from root to leaf on each side of the tree
 - While looking down, I want to find which side the start is and how far down
 - Take the max between the longest route on the side without the start + the distance to root and the distance from start to the bottom.
  */

const amountOfTime = function (root, start) {
  let time = 0;
  
  function dps(node) {
    if (!node) return 0;

    let leftDepth = dps(node.left);
    let rightDepth = dps(node.right);
    // This is infection
    if (node.val === start) {
      // Set time to the max time to bottom of subtree
      time = Math.max(leftDepth, rightDepth);
      return -1;
    }
    // No infection in subtree
    else if (leftDepth >= 0 && rightDepth >= 0) { 
      // Return the max time to bottom of subtree + connection to this node
      return Math.max(leftDepth, rightDepth) + 1;
    }
    // Infection in a child
    else {
      time = Math.max(time, Math.abs(leftDepth - rightDepth));
      return Math.min(leftDepth, rightDepth) - 1;
    }
  }

  dps(root);
  return time;
};


function testAmountOfTime1(testNum, tree, start, expected) { 
  let result = amountOfTime(tree, start);
  console.log(
    `Test ${testNum}: `,
    result,
    result === expected? " Pass ✅" : " Fail ❌"
  );
}

const tree1 = new TreeNode(
  1,
  new TreeNode(
    5,
    null,
    new TreeNode(
      4,
      new TreeNode(
        9,
      ),
      new TreeNode(
        2,
        null
      )
    )
  ),
  new TreeNode(
    3,
    new TreeNode(
      10
    ),
    new TreeNode(
      6
    )
  )
);
testAmountOfTime1(1, tree1, 3, 4);

const tree2 = new TreeNode(
  1
);
testAmountOfTime1(2, tree2, 1, 0);

const tree3 = new TreeNode(
  1,
  new TreeNode(
    2,
    new TreeNode(
      3,
      new TreeNode(
        4,
        new TreeNode(
          5,
          null
        )
      )
    )
  )
);
testAmountOfTime1(3, tree3, 3, 2);

const tree4 = new TreeNode(
  1,
  null,
  new TreeNode(
    2,
    new TreeNode(
      3,
      null,
      new TreeNode(
        5,
        null
      )
    ),
    new TreeNode(
      4
    )
  )
);
testAmountOfTime1(4, tree4, 4, 3);