use std::vec::Vec;

use crate::freq_tree::*;

use bitstream::{VecStream, BitReader};

/// Huffman Decoder struct, currently only used as placeholder struct
pub struct HuffmanDecoder {

}

impl HuffmanDecoder {
    /// Directly decodes a buffer encoded with the HuffmanEncoder. 
    pub fn decode(buf: Vec<u8>) -> Vec<u8> {
        let (ftree, mut original_size, encoded) = Self::parse_header(buf);
        let mut vs = VecStream::from_vec(encoded);
        let mut breader = BitReader::with_reader(&mut vs);

        let mut ret = Vec::<u8>::new();

        let mut at = &ftree;

        while original_size > 0 {
            let bit = breader.get_bit();
            if bit.is_none() {
                break;
            }

            if let FreqNodeData::Composit(c) = &at.data {
                match bit.unwrap() {
                    false => at = c.left.as_ref(),
                    true => at = c.right.as_ref()
                }
            }

            if let FreqNodeData::Value(v) = &at.data {
                ret.push(v.byte_val);
                at = &ftree;
                original_size -= 1;
            }

        }

        ret
    }

    fn parse_header(buf: Vec<u8>) -> (FreqTreeNode, u64, Vec<u8>) {
        let (header_size, orig_header) = Self::parse_32le_val(buf);
        let (orig_size, mut header) = Self::parse_64le_val(orig_header);

        let remaining = header.split_off(header_size as usize);

        (FreqTreeNode::decode(&header), orig_size, remaining)
    }

    fn parse_32le_val(mut buf: Vec<u8>) -> (u32, Vec<u8>) {
        let remaining = buf.split_off(std::mem::size_of::<u32>());
        let mut val_array = [0u8; std::mem::size_of::<u32>()];
        let val_bytes = &buf[..val_array.len()];
        val_array.copy_from_slice(val_bytes);
        let val = u32::from_le_bytes(val_array);
        (val, remaining)
    }

    fn parse_64le_val(mut buf: Vec<u8>) -> (u64, Vec<u8>) {
        let remaining = buf.split_off(std::mem::size_of::<u64>());
        let mut val_array = [0u8; std::mem::size_of::<u64>()];
        let val_bytes = &buf[..val_array.len()];
        val_array.copy_from_slice(val_bytes);
        let val = u64::from_le_bytes(val_array);
        (val, remaining)
    }
}
