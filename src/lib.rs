extern crate bitstream;

mod freq_tree;
mod huffman_encoder;
mod huffman_decoder;

pub use huffman_encoder::HuffmanEncoder;
pub use huffman_decoder::HuffmanDecoder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huffman_encoder_basic() {
        let s = "Hello, World!".to_owned();
        let mut h = HuffmanEncoder::new();

        h.add_chunk(s.as_bytes());

        let v = h.encode();

        println!("v: {:?}", v);

        let data = HuffmanDecoder::decode(v);
        let ds = String::from_utf8(data).unwrap();

        assert_eq!(s, ds);
    }
}
