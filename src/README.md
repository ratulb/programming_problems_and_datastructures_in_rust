## Programming problems & Data Structures In Rust

A collection of **Rust implementations** for common programming problems, interview questions, and fundamental data structures. This project serves as a resource for those looking to understand algorithmic problem-solving in Rust with detailed explanations and efficient implementations.

## ⚡ About the Project
This repository provides well-structured and optimized solutions to classical algorithmic problems using Rust. Each problem includes:
- **Clear Problem Descriptions**: Understanding the problem statement.
- **Rust Implementations**: Efficiently written, idiomatic Rust code.
- **Algorithmic Insights**: Explanation of the approach, trade-offs, and optimizations.
- **Test Cases**: Ensuring correctness and performance.

## ✨ Featured Problems & Implementations
The project covers a variety of topics, including:

### 🔢 **Array & Number Problems**
- Segregate negatives & positives
- Buy and sell stock once
- Contains duplicate
- Contains nearby duplicate
- Maximum subarray sum
- Two sum
- Maximum product subarray
- Product of array except self
- K nearest points from a given point

### 🔠 **String Problems**
- Segregate RGB characters
- Longest common prefix
- Longest common suffix

### 📈 **Dynamic Programming & Recursion**
- Longest increasing subsequence
- Number of ways to reach matrix end
- Palindrome partitioning
- N-Queens problem
- Subsets (backtracking, iterative, bit manipulation)
- Combination sum (multiple variants)

### 🔍 **Search & Sorting**
- Minimum in sorted rotated array
- Insert a new interval to a list of sorted intervals
- Search in a row and column-wise sorted matrix
- Search a target in a sorted rotated array

### 🌳 **Data Structures Implementations**
- **Binary Search Tree (BST)** with parent pointers
- **Trie (Prefix Tree)** for efficient string searching
- **Min Heap & Max Heap** for priority queue operations

## 🦀 Why Rust?
Rust provides **memory safety**, **performance**, and **concurrency** without the overhead of garbage collection. This makes it an excellent choice for implementing algorithmic solutions that require efficiency and reliability.

## 📜 Example: Binary Search Tree (BST) with Parent Pointers
A **BST (Binary Search Tree)** allows efficient searching, insertion, and deletion of elements. Our implementation uses **parent pointers** to facilitate operations such as node deletion efficiently.

### 🌲 Node Definition
```rust
struct Node<T: Ord + Default + Clone + std::fmt::Debug> {
    key: T,
    left: Option<Rc<RefCell<Tree<T>>>>,
    right: Option<Rc<RefCell<Tree<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}
```
### 🔧 Key Features
- **Uses `Rc<RefCell<Node<T>>`** for shared ownership and interior mutability.
- **Weak parent references** to prevent memory leaks.

## 🏔️ Example: Max Heap Implementation
A **Max Heap** ensures that the parent node is always larger than its children. It is useful for implementing priority queues.

```rust
#[derive(Debug)]
pub struct MaxHeap<T: Ord> {
    elements: Vec<T>,
}
```
### 🔹 Heap Operations
- **Insertion**: Adds an element and maintains heap property.
- **Deletion**: Removes the largest element and rebalances the heap.

## 💡 Who Is This For?
This project is perfect for:
- Rust developers looking to practice algorithms.
- Engineers preparing for coding interviews.
- Students learning data structures and algorithms.

## 🤝 Contributing
Contributions are welcome! Feel free to open issues, suggest improvements, or submit pull requests.

> **Note:** The source code of the repository can be found [here](https://github.com/ratulb/programming_problems_and_datastructures_in_rust) 

