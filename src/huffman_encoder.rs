use std::vec::Vec;
use std::collections::{LinkedList, HashMap, BinaryHeap, VecDeque};
use std::io::Write;

use crate::freq_tree::*;

use bitstream::{VecStream, BitWriter};

pub struct HuffmanEncoder {
    byte_counts: HashMap<u8, u64>,
    chunks: LinkedList<Vec<u8>>,
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

    pub fn encode(self) -> Vec<u8> {
        let ftree = self.build_freq_tree();
        let encoding_map = self.build_encoding_map(&ftree);
        let mut vs = VecStream::new();
        let mut bit_writer = BitWriter::with_writer(&mut vs);
        let mut orig_size: u64 = 0;

        for chunk in self.chunks {
            orig_size += chunk.len() as u64;
            for byte in chunk {
                bit_writer.write(&encoding_map.get(&byte).unwrap()).unwrap();
            }
        }

        drop(bit_writer);

        let mut ftree_serialized = ftree.encode();
        let mut orig_header = orig_size.to_le_bytes().to_vec();
        let mut header_size = (ftree_serialized.len() as u32).to_le_bytes().to_vec();
        let mut huff_data = vs.into_vec();

        let mut encoded = Vec::<u8>::new();
        encoded.append(&mut header_size);
        encoded.append(&mut orig_header);
        encoded.append(&mut ftree_serialized);
        encoded.append(&mut huff_data);
        encoded
    }

    fn build_freq_tree(&self) -> Box<FreqTreeNode> {
        let mut heap: BinaryHeap<FreqTreeNode> = BinaryHeap::new();

        // add all of the values to the heap as Value nodes
        for (k, v) in &self.byte_counts {
            heap.push(FreqTreeNode {data : FreqNodeData::Value( FreqTreeVal {byte_val: *k, occures: *v} ) });
        }

        while heap.len() > 1 {
            let left: FreqTreeNode = heap.pop().unwrap();
            let right: FreqTreeNode = heap.pop().unwrap();

            let total = left.get_weight() + right.get_weight();

            let composit = FreqTreeNode { data : FreqNodeData::Composit( FreqTreeComposit {
                occures: total,
                left: Box::new(left),
                right: Box::new(right),
            })};
            heap.push(composit);
        }

        Box::new(heap.pop().unwrap())
    }

    fn build_encoding_map(&self, ftree: &Box<FreqTreeNode>) -> HashMap<u8, Vec<u8>> {
        struct QueueNode<'a> {
            node: &'a Box<FreqTreeNode>,
            bits: Vec<u8>,
        }
        let mut tqueue = VecDeque::<QueueNode>::new();
        let mut encoding_map = HashMap::<u8, Vec<u8>>::new();

        tqueue.push_back(QueueNode{
            node: ftree,
            bits: Vec::new()
        });

        loop {
            let node = tqueue.pop_front();
            match node {
                Some(n) => {
                    match &n.node.data {
                        FreqNodeData::Composit(c) => {
                            let mut left_vec = n.bits.clone();
                            left_vec.push(0);
                            tqueue.push_back(QueueNode {
                                node: &c.left,
                                bits: left_vec,
                            });

                            let mut right_vec = n.bits.clone();
                            right_vec.push(1);
                            tqueue.push_back(QueueNode {
                                node: &c.right,
                                bits: right_vec,
                            });
                        },
                        FreqNodeData::Value(v) => {
                            encoding_map.insert(v.byte_val, n.bits);
                        }
                    }
                },
                None => break
            }
        }

        encoding_map
    }
}

