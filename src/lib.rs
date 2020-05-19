use std::vec::Vec;
use std::collections::{LinkedList, HashMap, BinaryHeap};
use std::cmp::Ordering;

extern crate bitstream;

pub struct HuffmanEncoder {
    byte_counts: HashMap<u8, u64>,
    chunks: LinkedList<Vec<u8>>,
}

struct FreqTreeNode {
    data: FreqNodeData,
}

enum FreqNodeData {
    Composit (FreqTreeComposit),
    Value (FreqTreeVal),
}

struct FreqTreeVal {
    byte_val: u8,
    occures: u64
}

struct FreqTreeComposit {
    occures: u64,
    left: Box<FreqTreeNode>,
    right: Box<FreqTreeNode>,
}

impl FreqTreeNode {
    pub fn get_weight(&self) -> u64 {
        use FreqNodeData::*;
        match self.data {
            Composit(c) => c.occures,
            Value(v) => v.occures,
        }
    }
}

impl PartialEq for FreqTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_weight() == other.get_weight()
    }
}

impl Eq for FreqTreeNode {}

impl PartialOrd for FreqTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FreqTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_weight = self.get_weight();
        let other_weight = other.get_weight();
        if my_weight < other_weight {
            Ordering::Greater
        } else if my_weight > other_weight {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl HuffmanEncoder {
    pub fn new() -> HuffmanEncoder {
        HuffmanEncoder {
            byte_counts: HashMap::new(),
            chunks: LinkedList::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: &[u8]) {

        for c in chunk {
            if self.byte_counts.contains_key(c) {
                *self.byte_counts.get_mut(c).unwrap() += 1;
            } else {
                self.byte_counts.insert(*c, 1);
            }
        }

        self.chunks.push_back(chunk.to_vec());
    }

    pub fn encoder(mut self) -> Vec<u8> {

    }

    fn build_freq_tree(&self) -> Box<FreqTreeNode> {
        use FreqNodeData::*;
        let mut heap: BinaryHeap<FreqTreeNode> = BinaryHeap::new();

        // add all of the values to the heap as Value nodes
        for (k, v) in self.byte_counts {
            heap.push(FreqTreeNode {data : Value( FreqTreeVal {byte_val: k, occures: v} ) });
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();

            let total = left.get_weight() + right.get_weight();

            let composit = FreqTreeNode { data : Composit( FreqTreeComposit {
                occures: total,
                left: Box::new(left),
                right: Box::new(right),
            })};
            heap.push(composit);
        }

        Box::new(heap.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
