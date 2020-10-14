/**
 * 莫里斯遍历
 * 二叉树的遍历算法
 * 参考   https://www.cnblogs.com/AnnieKim/archive/2013/06/15/morristraversal.html
 * @description 非递归、不用栈、空间复杂度O(1)
 */

interface NodeInTree {
  val: number;
  left: NodeInTree | null;
  right: NodeInTree | null;
}
type TreeNode = NodeInTree | null;

class NodeInTree implements NodeInTree {
  constructor(val: number) {
    this.left = null;
    this.right = null;
    this.val = val;
  }
}

/**
 * 中序遍历
 * @param root 根结点
 */
const inorderMorrisTraversal = (root: TreeNode) => {
  let cur = root;
  let prev: TreeNode = null;

  while (cur !== null) {
    if (cur.left === null) {
      console.log("left null: ", cur.val);
      cur = cur.right;
    } else {
      prev = cur.left;
      while (prev.right !== null && prev.right !== cur) {
        prev = prev.right;
      }
      if (prev.right === null) {
        prev.right = cur;
        cur = cur.left;
      } else {
        prev.right = null;
        console.log("last if: ", cur.val);
        cur = cur.right;
      }
    }
  }
};

/**
 * 前序遍历
 * @param root 节点
 */
const preorderMorrisTraversal = (root: TreeNode) => {
  let cur = root;
  let prev: TreeNode = null;

  while (cur !== null) {
    if (cur.left === null) {
      console.log("let null: ", cur.val);
      cur = cur.right;
    } else {
      prev = cur.left;
      while (prev.right !== null && prev.right !== cur) {
        prev = prev.right;
      }

      if (prev.right === null) {
        console.log("last if: ", cur.val);
        prev.right = cur;
        cur = cur.left;
      } else {
        prev.right = null;
        cur = cur.right;
      }
    }
  }
};

const reverse = (from: TreeNode, to: TreeNode) => {
  if (from === to || from === null) {
    return;
  }
  let x = from;
  let y = from.right;
  let z: TreeNode;
  while (true) {
    if (y && y.right) {
      z = y.right;
      y.right = x;
      x = y;
      y = z;
    }
    if (x === to) {
      break;
    }
  }
};
const printReverse = (from: TreeNode, to: TreeNode) => {
  reverse(from, to);
  let p = to;
  while (true) {
    console.log("printReverse: ", p && p.val);
    if (p === from) {
      break;
    }
    p = p && p.right;
  }
};
const postorderMorrisTraversal = (root: TreeNode) => {
  let dump = new NodeInTree(0); // 虚拟节点
  dump.left = root;
  let cur: TreeNode = dump;
  let prev: TreeNode = null;

  while (cur !== null) {
    if (cur.left === null) {
      // console.log("left null: ", cur.val);
      cur = cur.right;
    } else {
      prev = cur.left;
      while (prev.right !== null && prev.right !== cur) {
        prev = prev.right;
      }
      if (prev.right === null) {
        prev.right = cur;
        cur = cur.left;
      } else {
        // console.log("last if: ", cur.val);
        printReverse(cur.left, prev)
        prev.right = null;
        cur = cur.right;
      }
    }
  }
};

const nodes: NodeInTree = {
  val: 6,
  left: {
    val: 2,
    left: {
      val: 1,
      left: null,
      right: null,
    },
    right: {
      val: 4,
      left: {
        val: 3,
        left: null,
        right: null,
      },
      right: {
        val: 5,
        left: null,
        right: null,
      },
    },
  },
  right: {
    val: 7,
    left: null,
    right: {
      val: 9,
      left: {
        val: 8,
        left: null,
        right: null,
      },
      right: null,
    },
  },
};

inorderMorrisTraversal(nodes);
console.log("---------------");
preorderMorrisTraversal(nodes);
console.log("---------------");
postorderMorrisTraversal(nodes);
