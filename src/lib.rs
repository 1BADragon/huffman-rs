extern crate bitstream;

mod freq_tree;
mod huffman_encoder;
mod huffman_decoder;

pub use huffman_encoder::HuffmanEncoder;
pub use huffman_decoder::HuffmanDecoder;

#[cfg(test)]
mod tests {
    extern crate rand;

    use rand::Rng;
    use rand::distributions::Alphanumeric;
    use super::*;

    #[test]
    fn huffman_encoder_basic() {
        let s = "Hello, World!".to_owned();
        let mut h = HuffmanEncoder::new();

        h.add_chunk(s.as_bytes());

        let v = h.encode();

        let data = HuffmanDecoder::decode(v);
        let ds = String::from_utf8(data).unwrap();

        assert_eq!(s, ds);
    }

    #[test]
    fn huffman_large_buffer() {
        let s = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4096)
            .collect::<String>();

        let mut h = HuffmanEncoder::new();
        h.add_chunk(s.as_bytes());
        let v = h.encode();

        let data = HuffmanDecoder::decode(v);
        let ds = String::from_utf8(data).unwrap();


        assert_eq!(s, ds);
    }
}
