### 1. Full Binary Tree

Every node has either 0 or 2 children.

```text
       1
      / \
     2   3
    / \   \
   4   5   6
```

### 2. Complete Binary Tree

All levels are full except possibly the last, which fills left-to-right.

```text
       1
      / \
     2   3
    / \  /
   4  5 6
```

### 3. Perfect Binary Tree

Every internal node has 2 children and all leaves are at the same level.

```text
       1
      / \
     2   3
    / \ / \
   4  5 6  7
```

### 4. Balanced Binary Tree

The heights of left and right subtrees stay roughly equal.

```text
       1
      / \
     2   3
    / \   \
   4   5   6
```

### 5. Skewed Binary Tree

Each node has only one child, resembling a linked list.

```text
1
 \
  2
   \
    3
     \
      4
```

### 6. Binary Search Tree (BST)

Left subtree values are smaller and right subtree values are larger.

```text
           8
      /        \
     3          10
    / \           \
   1   6          14
      / \        /
     4   7     13
```

### 7. AVL Tree

A self-balancing BST where subtree heights differ by at most 1.

```text
       4
      / \
     2   6
    / \ / \
   1  3 5  7
```

### 8. Red-Black Tree

A self-balancing BST where nodes have red/black colors enforcing balance rules.

```text
       4(B)
      /    \
   2(R)    6(R)
   / \     / \
 1(B)3(B) 5(B)7(B)
```

### 9. B-Tree

A multi-way balanced search tree designed for efficient disk/database access. B-Tree notation, [10 | 20] represents one node containing multiple sorted keys:

```text
          [10 | 20]
         /    |    \
     [1|5] [12|15] [25|30]
```

### 10. B+ Tree

Internal nodes store keys, while actual records are stored in linked leaf nodes.

```text
          [10 | 20]
         /    |    \
     [1|5] [10|15] [20|25]
       └──────┬──────┘
          linked leaves
```

### 11. Heap

A complete binary tree satisfying the min-heap or max-heap property.

### 11. Heap

A **heap** is a **complete binary tree** that follows a specific ordering rule.

There are two common types:

**Max-Heap:** parent ≥ children

```text
       50
      /  \
    30    40
   / \    /
 10  20  35
```

**Min-Heap:** parent ≤ children

```text
       10
      /  \
    20    15
   / \    /
 30  40  35
```

### Key feature

The **root always contains the minimum or maximum element**, depending on the heap type.

This makes heaps particularly useful for implementing **priority queues**.


### 12. Trie

Stores strings character-by-character along paths.

```text
       root
        |
        c
        |
        a
       / \
      t   r
      |   |
      e   d
```

### 13. Segment Tree

Represents ranges of an array for efficient range queries and updates.

```text
          [0..7]
         /     \
      [0..3]  [4..7]
      /   \    /   \
   [0..1][2..3][4..5][6..7]
```

### 14. Fenwick Tree

Uses a tree-like array structure for efficient prefix-sum updates and queries.

```text
        8
       /
      4
     /
    2
   /
  1
```

### 15. Splay Tree

Recently accessed nodes are moved toward the root through rotations.

```text
       5
      / \
     3   8
    / \
   1   4
```
