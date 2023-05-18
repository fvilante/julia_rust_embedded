#[derive(Debug, PartialEq)]
pub enum RingBufferError {
    BufferFull,
    BufferEmpty,
}

/// TODO: Substitute and reuse some already existent data type from [`heapless`] crate
pub struct RingBuffer<T: Copy, const SIZE: usize> {
    buffer: [T; SIZE],
    write_index: usize,
    read_index: usize,
    is_full: bool,
}

impl<T: Copy, const SIZE: usize> RingBuffer<T, SIZE> {
    pub const fn new(buffer: [T; SIZE]) -> Self {
        Self {
            buffer,
            write_index: 0x00,
            read_index: 0x00,
            is_full: false,
        }
    }

    fn get_next_bounded_index(&self, index: usize) -> usize {
        let mut new_index = index + 1;
        let is_inside_bound = new_index < self.buffer.len();
        if is_inside_bound {
            new_index
        } else {
            //overflow index, then reset index
            new_index = 0;
            new_index
        }
    }

    pub fn is_empty(&self) -> bool {
        let is_empty = (self.read_index == self.write_index) && (self.is_full == false);
        is_empty
    }

    pub fn is_full(&self) -> bool {
        self.is_full
    }

    pub fn write(&mut self, data: T) -> Result<(), RingBufferError> {
        // check if it is full
        if self.is_full() {
            return Err(RingBufferError::BufferFull);
        } else {
            // write and advance pointer
            self.buffer[self.write_index] = data;
            self.write_index = self.get_next_bounded_index(self.write_index);
            // set 'buffer full' flag if necessary
            let is_full_after = self.write_index == self.read_index;
            if is_full_after {
                self.is_full = true;
            };
            return Ok(());
        }
    }

    pub fn read(&mut self) -> Result<T, RingBufferError> {
        // if buffer empty emit error
        if self.is_empty() {
            return Err(RingBufferError::BufferEmpty);
        } else {
            // read and advance pointer
            let data = self.buffer[self.read_index];
            self.read_index = self.get_next_bounded_index(self.read_index);
            // after a sucessful read, buffer will never be full
            self.is_full = false;
            return Ok(data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_write_read_one_byte() {
        let mut ring = RingBuffer::new([0x00; 3]);
        // can write and read one byte
        let probe = 0x77;
        let expected = probe;
        ring.write(probe).unwrap();
        let actual = ring.read().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_signals_buffer_empty() {
        let mut ring = RingBuffer::new([0x00; 3]);
        let actual = ring.read().unwrap_err();
        assert_eq!(RingBufferError::BufferEmpty, actual)
    }

    #[test]
    fn it_signals_buffer_full() {
        let mut ring = RingBuffer::new([0x00; 3]);
        ring.write(0x01).unwrap();
        ring.write(0x02).unwrap();
        ring.write(0x03).unwrap();
        let actual = ring.write(0x04).unwrap_err();
        assert_eq!(RingBufferError::BufferFull, actual)
    }

    #[test]
    fn it_not_signals_buffer_full() {
        let mut ring = RingBuffer::new([0x00; 3]);
        ring.write(0x01).unwrap();
        ring.read().unwrap();
        ring.write(0x02).unwrap();
        ring.write(0x03).unwrap();
        let actual = ring.write(0x04).unwrap();
        assert_eq!((), actual)
    }

    #[test]
    fn it_clears_buffer_full_signal() {
        let mut ring = RingBuffer::new([0x00; 3]);
        ring.write(0x01).unwrap();
        ring.write(0x02).unwrap();
        ring.write(0x03).unwrap();
        let is_full = ring.write(0x04).unwrap_err();
        ring.read().unwrap();
        let is_not_full = ring.write(0x05).unwrap();
        assert_eq!(RingBufferError::BufferFull, is_full);
        assert_eq!((), is_not_full);
    }

    #[test]
    fn it_write_and_ready_dynamically() {
        // prepare
        let mut ring = RingBuffer::new([0x00; 3]);
        let probe = [0, 1, 2, 3, 4, 5, 6];
        // act
        ring.write(probe[0]).unwrap();
        let a0 = ring.read().unwrap();
        ring.write(probe[1]).unwrap();
        ring.write(probe[2]).unwrap();
        let a1 = ring.read().unwrap();
        let a2 = ring.read().unwrap();
        let empty = ring.read().unwrap_err();
        ring.write(probe[3]).unwrap();
        ring.write(probe[4]).unwrap();
        ring.write(probe[5]).unwrap();
        let full = ring.write(probe[4]).unwrap_err();
        let a3 = ring.read().unwrap();
        let a4 = ring.read().unwrap();
        let a5 = ring.read().unwrap();
        let empty_again = ring.read().unwrap_err();
        ring.write(probe[6]).unwrap();
        let a6 = ring.read().unwrap();
        //check
        assert_eq!(probe[0], a0);
        assert_eq!(probe[1], a1);
        assert_eq!(probe[2], a2);
        assert_eq!(RingBufferError::BufferEmpty, empty);
        assert_eq!(RingBufferError::BufferFull, full);
        assert_eq!(probe[3], a3);
        assert_eq!(probe[4], a4);
        assert_eq!(probe[5], a5);
        assert_eq!(RingBufferError::BufferEmpty, empty_again);
        assert_eq!(probe[6], a6);
    }
}
