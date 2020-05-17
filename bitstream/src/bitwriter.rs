use std::io::{Write, Result};
use std::vec::Vec;

pub struct BitWriter<'a>
{
    writer: &'a mut (dyn Write + 'a),
    buffer: Vec<u8>,
    write_threshold: usize,
    cur_byte_loc: u8,
    cur_byte: u8,
}


impl<'a> BitWriter<'a> {
    pub fn with_writer(writer: &'a mut dyn Write) -> BitWriter {
        BitWriter {
            writer: writer,
            buffer: Vec::with_capacity(128),
            write_threshold: 128,
            cur_byte_loc: 0,
            cur_byte: 0,
        }
    }

    pub fn add_bit(&mut self, val: bool) -> Result<()>{
        let mask: u8 = (val as u8) << (7 - self.cur_byte_loc);

        self.cur_byte |= mask;
        self.cur_byte_loc += 1;

        if self.cur_byte_loc == 8 {
            self.buffer.push(self.cur_byte);
        }

        if self.buffer.len() >= self.write_threshold {
            self.flush()?;
        }

        Ok(())
    }
}

impl<'a> Write for BitWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        for b in buf {
            self.add_bit(*b != 0)?;
        }
        return Ok(buf.len());
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.write_all(&self.buffer)?;
        self.buffer.clear();
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io::*;
    use super::crate::VecStream;
    use super::*;

    #[test]
    fn BitWriter_test() {
        let vs = VecStream::new();
        let bt = BitWriter::with_writer(&mut vs);


    }
}
