use std::io::{Read,Result};
use std::option::Option;
use std::vec::Vec;

pub struct BitReader<'a> {
    reader: &'a mut (dyn Read + 'a),
    buffer: Vec<u8>,
    read_amount: usize,
    cur_byte_loc: u8,
}

impl<'a> BitReader<'a> {
    pub fn with_reader(reader: &'a mut dyn Read) -> BitReader {
        BitReader {
            reader: reader,
            buffer: Vec::new(),
            read_amount: 128,
            cur_byte_loc: 0,
        }
    }

    pub fn get_bit(&mut self) -> Option<bool> {
        if self.buffer.len() == 0 && self.cur_byte_loc == 0 {
            self.buffer.resize(self.read_amount, 0);
            let size = self.reader.read(&mut self.buffer).unwrap();
            if size == 0 {
                return None;
            }
            self.buffer.resize(size, 0);
        }

        let mask = (1 as u8) << (7 - self.cur_byte_loc);
        let val = self.buffer[0] & mask;
        
        self.cur_byte_loc += 1;

        if self.cur_byte_loc == 8 {
            self.cur_byte_loc = 0;
            self.buffer = self.buffer.split_off(1);
        }

        Some(val != 0)
    }

    pub fn set_read_amount(&mut self, val: usize) {
        self.read_amount = val;
    }

    pub fn into_remaining(self) -> Vec<u8> {
        self.buffer
    }
}

impl<'a> Read for BitReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut index = 0;

        while index < buf.len() {
            match self.get_bit() {
                Some(b) => {
                    buf[index] = b as u8;
                    index += 1;
                },
                None => {
                    break;
                }
            }
        }
        return Ok(index);
    }
}

#[cfg(test)]
mod tests {
    use std::io::*;
    use crate::VecStream;
    use super::*;

    #[test]
    fn bit_reader_basic() {
        let v: Vec<u8> = vec![0b10010110];

        let mut vs = VecStream::from_vec(v);
        let mut br = BitReader::with_reader(&mut vs);

        let mut out_v = Vec::<u8>::new();
        out_v.resize(8, 0);
        br.read(&mut out_v).unwrap();

        assert_eq!(out_v[0], 1);
        assert_eq!(out_v[1], 0);
        assert_eq!(out_v[2], 0);
        assert_eq!(out_v[3], 1);
        assert_eq!(out_v[4], 0);
        assert_eq!(out_v[5], 1);
        assert_eq!(out_v[6], 1);
        assert_eq!(out_v[7], 0);
    }
}
