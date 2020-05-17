use std::io::{Read,Write,Seek,SeekFrom,Result};
use std::vec::Vec;
use std::cmp::{min, max};

///
/// An in-memory IO Buffer that can be used similar to a file but without any actual
/// io. All io operations are performed in memory reads can be seeked and writes will
/// enlarge the in memory buffer.
///
#[derive(Clone)]
pub struct VecStream {
    buffer: Vec<u8>,
    read_pos: usize,
}

impl VecStream {
    /// Create a new VecStream with an empty buffer. This will appear as a file of
    /// size 0 with the Seek Impl.
    pub fn new() -> VecStream {
        VecStream {
            buffer: Vec::<u8>::new(),
            read_pos: 0
        }
    }
}

impl Read for VecStream {
    /// Read from the internal buffer. Cannot read past buffer end. If attempting to read
    /// past the buffer a return value of the bytes available will be returned. Will
    /// return 0 if the read cursor is at the end of the buffer.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_size = min(buf.len(), self.buffer.len() - self.read_pos);

        for i in 0..read_size {
            buf[i] = self.buffer[i + self.read_pos];
        }

        self.read_pos += read_size;

        return Ok(read_size);
    }
}

impl Write for VecStream {
    /// Write to the internal buffer. In the current implmentation all write will extend the
    /// buffer. This behavior can change if the need for the ability to seek the write
    /// cursor occurs.
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.extend_from_slice(buf);
        return Ok(buf.len());
    }

    /// The flush() function is a no-op due to the buffer staying in memory.
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for VecStream {
    /// Seeks the read cursor to a given location.
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(x) => self.read_pos = min(self.buffer.len(), x as usize),
            SeekFrom::End(x) => {
                if x >= 0 {
                    self.read_pos = self.buffer.len();
                } else {
                    self.read_pos = max(0, self.buffer.len() as i64 + x) as usize;
                }
            },
            SeekFrom::Current(x) => {
                if x >= 0 {
                    self.read_pos = min(self.buffer.len() as i64, self.read_pos as i64 + x) as usize;
                } else {
                    self.read_pos = max(0, self.read_pos as i64 + x) as usize;
                }
            }
        };

        Ok(self.read_pos as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read,Write,Seek,SeekFrom};
    use super::VecStream;

    #[test]
    fn vecstream_basic() {
        let mut vs = VecStream::new();

        vs.write(b"hello").unwrap();
        let mut s = String::new();
        vs.read_to_string(&mut s).unwrap();

        assert_eq!(s, "hello");
    }

    #[test]
    fn vecstream_multipart() {
        let mut vs = VecStream::new();

        vs.write(b"hello").unwrap();
        vs.write(b" world").unwrap();

        let mut s = String::new();
        vs.read_to_string(&mut s).unwrap();

        assert_eq!(s, "hello world");
    }

    #[test]
    fn vecstream_multiread_multiwrite() {
        let mut vs = VecStream::new();

        vs.write(b"hello").unwrap();
        let mut s = String::new();
        vs.read_to_string(&mut s).unwrap();

        assert_eq!(s, "hello");

        s.clear();
        vs.write(b"world").unwrap();
        vs.read_to_string(&mut s).unwrap();
        assert_eq!(s, "world");
    }

    #[test]
    fn vecstream_seek() {
        let mut vs = VecStream::new();
        vs.write(b"hello world").unwrap();
        let mut s = String::new();
        vs.read_to_string(&mut s).unwrap();

        assert_eq!(s, "hello world");

        vs.seek(SeekFrom::Start(0));
        s.clear();
        vs.read_to_string(&mut s).unwrap();

        assert_eq!(s, "hello world");
    }
}
