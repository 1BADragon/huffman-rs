use std::cmp::Ordering;

use bitstream::*;

pub struct FreqTreeNode {
    pub data: FreqNodeData,
}

pub enum FreqNodeData {
    Composit (FreqTreeComposit),
    Value (FreqTreeVal),
}

pub struct FreqTreeVal {
    pub byte_val: u8,
    pub occures: u64
}

pub struct FreqTreeComposit {
    pub occures: u64,
    pub left: Box<FreqTreeNode>,
    pub right: Box<FreqTreeNode>,
}

impl FreqTreeNode {
    pub fn get_weight(&self) -> u64 {
        use FreqNodeData::*;
        match &self.data {
            Composit(c) => c.occures,
            Value(v) => v.occures,
        }
    }

    pub fn decode(data: &[u8]) -> FreqTreeNode {
        let mut vs = VecStream::from_vec(data.to_owned());
        let mut br = BitReader::with_reader(&mut vs);

        Self::decode_node(&mut br)
    }

    fn decode_node(reader: &mut BitReader) -> FreqTreeNode {
        if reader.get_bit().unwrap() {
            FreqTreeNode { data: FreqNodeData::Value ( FreqTreeVal { byte_val: reader.get_byte().unwrap(), occures: 0 } ) }
        } else {
            FreqTreeNode { data: FreqNodeData::Composit (
                FreqTreeComposit {
                    occures: 0,
                    left: Box::new(Self::decode_node(reader)),
                    right: Box::new(Self::decode_node(reader)),
                }
            )}
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut vs = VecStream::new();
        let mut bw = BitWriter::with_writer(&mut vs);

        Self::encode_node(self, &mut bw);

        drop(bw);
        vs.into_vec()
    }

    fn encode_node(node: &FreqTreeNode, writer: &mut BitWriter) {
        match &node.data {
            FreqNodeData::Composit(c) => {
                writer.add_bit(false).unwrap();
                Self::encode_node(c.left.as_ref(), writer);
                Self::encode_node(c.right.as_ref(), writer);
            },
            FreqNodeData::Value(v) => {
                writer.add_bit(true).unwrap();
                writer.add_byte(v.byte_val).unwrap();
            }
        }
    }
}

impl PartialEq for FreqTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) != Ordering::Equal
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
