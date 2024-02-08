// 1. Check whether a given string is a palindrome or not
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// 2. Find the index of the first occurrence of a given number in a sorted array
fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// 3. Find the shortest word in a string of words
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

// Check whether a given number is prime or not
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// 4. Find the median of a sorted array of integers
fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// 5. Find the longest common prefix of a set of strings
fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

// 6. Find the kth smallest element in a given array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted = arr.to_vec();
        sorted.sort();
        Some(sorted[k - 1])
    } else {
        None
    }
}

// 7. Find the maximum depth of a binary tree
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => 1 + max_depth(node.left.as_ref()).max(max_depth(node.right.as_ref())),
        None => 0,
    }
}

// 8. Reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 9. Check if a number is prime
fn is_prime_rust(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// 10. Merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);
    result
}

// 11. Find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    // Testing the functions
    println!("Is 'racecar' a palindrome? {}", is_palindrome("racecar"));
    println!("Index of first occurrence of 5: {:?}", find_first_occurrence(&[1, 2, 3, 4, 5, 5, 6], 5));
    println!("Shortest word in 'hello world': {:?}", shortest_word("hello world"));
    println!("Is 17 prime? {}", is_prime(17));
    println!("Median of [1, 3, 5, 7, 9]: {}", find_median(&[1, 3, 5, 7, 9]));
    println!("Longest common prefix: {}", longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
    println!("3rd smallest element: {:?}", kth_smallest(&[4, 2, 5, 1, 3], 3));
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));

    println!("Max depth of binary tree: {}", max_depth(root.as_ref()));
    println!("Reverse of 'rust': {}", reverse_string("rust"));
    println!("Is 19 prime in Rust? {}", is_prime_rust(19));
    println!("Merged sorted arrays: {:?}", merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6]));
    println!("Maximum subarray sum: {}", max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    
}
