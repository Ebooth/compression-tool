use std::fs;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug)]
struct TreeNode {
    frequency: u64,
    char: Option<char>,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency && self.left == other.left && self.right == other.right
    }
}
impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.frequency.cmp(&self.frequency))
    }
}
impl Eq for TreeNode {}
impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.frequency.cmp(&self.frequency);
    }
}
fn main() {
    let content = read_file("tests/gutenberg.txt").expect("could not read the file");
    let counter = count_characters_frequency(&content);
    println!("counter : {counter:?} \n");
    let root = build_huffman_tree(&counter);
    println!("{root:?}");
}

fn count_characters_frequency(text: &str) -> HashMap<char, u64> {
    let mut counter = HashMap::new();
    text.chars().for_each(|c| {
        *counter.entry(c).or_insert(0) += 1;
    });
    return counter;
}

fn read_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn build_huffman_tree(counter: &HashMap<char, u64>) -> TreeNode {
    let mut heap = BinaryHeap::from_iter(counter.iter().map(|(&key, &val)| {
        return TreeNode {
            frequency: val,
            char: Some(key),
            left: None,
            right: None,
        };
    }));

    while heap.len() > 1 {
        let t1 = heap.pop().unwrap();
        let t2 = heap.pop().unwrap();
        let parent = TreeNode {
            frequency: t1.frequency + t2.frequency,
            char: None,
            left: Some(Box::new(t1)),
            right: Some(Box::new(t2)),
        };
        heap.push(parent);
    }

    return heap.pop().unwrap();
}
