use std::io::{Write, Result};
use std::vec::Vec;
use std::ops::Drop;


/// A write()able type that will translate bytes written into bits. This struct can 
/// be used as an implace writer for other IO writer types, (such as a File or TcpSocket).
/// 
/// All bytes written using this struct are translated into either a 0 bit if the byte 
/// value is 0 or a 1 bit otherwise. This struct requires a reference to an IO writer
/// instead of a move so the original Writer can be used for its other functionalities. 
///
/// The BitWriter struct uses an internal buffer to reduce the amount of calles to underling
/// Writer struct. This write_threshold has a starting value of 128 bytes but can be altered
/// with the set_write_threshold() method. The flush() method can be used to write the 
/// current contents of the buffer to the underlying Writer. Any uncompleted bytes are 
/// padded out with 0 bits. A call to flush for the BitWrite does not call flush on the 
/// underlying Writer. 
pub struct BitWriter<'a>
{
    writer: &'a mut (dyn Write + 'a),
    buffer: Vec<u8>,
    write_threshold: usize,
    cur_byte_loc: u8,
    cur_byte: u8,
}


impl<'a> BitWriter<'a> {
    /// Create a new BitWriter struct with the given writer as the endpoint of the BitWriter.
    /// The lifetime of the passed writer must exceed the lifetime of the BitWriter itsself.
    pub fn with_writer(writer: &'a mut dyn Write) -> BitWriter {
        BitWriter {
            writer: writer,
            buffer: Vec::with_capacity(128),
            write_threshold: 128,
            cur_byte_loc: 0,
            cur_byte: 0,
        }
    }

    /// Adds a single bit to the BitWriter
    pub fn add_bit(&mut self, val: bool) -> Result<()>{
        let mask: u8 = (val as u8) << (7 - self.cur_byte_loc);

        self.cur_byte |= mask;
        self.cur_byte_loc += 1;

        if self.cur_byte_loc == 8 {
            self.buffer.push(self.cur_byte);
            self.cur_byte_loc = 0;
            self.cur_byte = 0;
        }

        if self.buffer.len() >= self.write_threshold {
            self.flush()?;
        }

        Ok(())
    }

    /// Add a byte to the BitWriter as 8 bits ( as apposed to a single bit )
    pub fn add_byte(&mut self, val: u8) -> Result<()> {
        for i in (0..8).rev() {
            let mask = 1 << i;
            self.add_bit((val & mask) != 0)?;
        }

        Ok(())
    }

    /// Sets the write threshold on the BitWriter to be val bytes. When the write threshold
    /// has been reached on the internal buffer the BitWriter will call write on the 
    /// underlying Writer.
    pub fn set_write_threshold(&mut self, val: usize) {
        self.write_threshold = val;
    }
}

impl<'a> Write for BitWriter<'a> {
    /// Writes bits to the underlying Writer. For the slice that is passed in, each byte is
    /// interpreted as either a 0 bit for a 0 byte value and a 1 bit for a non-zero byte 
    /// value.
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        for b in buf {
            self.add_bit(*b != 0)?;
        }
        return Ok(buf.len());
    }

    /// Flushes the internal buffer to the underlying Writer struct. If the buffer does
    /// no contain a complete byte then the last byte is padded out with 0 bits. Calling
    /// flush does not call flush on the underlying device. 
    fn flush(&mut self) -> Result<()> {
        if self.cur_byte_loc > 0 {
            self.buffer.push(self.cur_byte);
            self.cur_byte_loc = 0;
            self.cur_byte = 0;
        }
        
        self.writer.write_all(&self.buffer)?;
        self.buffer.clear();
        Ok(())
    }
}

impl<'a> Drop for BitWriter<'a> {
    /// When the BitWriter is dropped, the internal buffer is flushed to the underlying 
    /// Writer struct. 
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::io::*;
    use crate::VecStream;
    use super::*;

    #[test]
    fn bit_writer_basic() {
        let mut vs = VecStream::new();
        let mut bt = BitWriter::with_writer(&mut vs);
        
        bt.add_bit(true).unwrap();
        bt.add_bit(false).unwrap();
        bt.add_bit(true).unwrap();
        bt.add_bit(true).unwrap();
        bt.flush().unwrap();

        drop(bt);
        let v = vs.into_vec();

        assert_eq!(v[0], 0b10110000);
    }

    #[test]
    fn bit_writer_multibyte() {
        let bit_arr: [u8; 16] = [1,0,0,1,0,0,1,0,0,0,1,1,0,0,1,0];
        let mut vs = VecStream::new();
        let mut bt = BitWriter::with_writer(&mut vs);

        bt.write(&bit_arr).unwrap();
        drop(bt);
    
        let v = vs.into_vec();

        println!("v: {:?}", v);

        assert_eq!(v[0], 0b10010010);
        assert_eq!(v[1], 0b00110010);
    }
}
