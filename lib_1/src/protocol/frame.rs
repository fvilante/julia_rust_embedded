use super::common::StartByte;


#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Frame<const SIZE: usize> {
    pub start_byte: StartByte,
    pub payload: [u8; SIZE],
}

impl<const SIZE: usize> Frame<SIZE> {
    
    pub const fn new(start_byte: StartByte, payload: [u8; SIZE]) -> Self {
        Self {
            start_byte,
            payload,
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frame_works() {
        //prepare
        let expected: [u8; 4] = [1,2,3,4];
        let start_byte = StartByte::STX;
        //act
        let frame = Frame::new(start_byte, expected);
        //check
        assert_eq!(expected, frame.payload);
        assert_eq!(start_byte, frame.start_byte);
    }
}



