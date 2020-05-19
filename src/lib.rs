use std::vec::Vec;
use std::collections::{LinkedList, HashMap};

extern crate bitstream;

pub struct Huffman {
    byte_counts: HashMap<u8, usize>,
    chunks: LinkedList<Vec<u8>>,
}

struct FreqTree {

}

impl Huffman {
    pub fn new() -> Huffman {
        Huffman {
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

    pub fn compress(mut self) -> Vec<u8> {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
