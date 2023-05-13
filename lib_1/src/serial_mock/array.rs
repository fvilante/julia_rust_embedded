pub struct Array<T: Copy, const SIZE: usize> {
    array: [T; SIZE],
}

impl<T: Copy, const SIZE: usize> Array<T, SIZE> {
    pub const fn from_array(init: [T; SIZE]) -> Array<T, SIZE> {
        Self { array: init }
    }

    pub unsafe fn get_from_index(&self, index: usize) -> T {
        self.array[index]
    }

    pub unsafe fn put_to_index(&mut self, index: usize, value: T) {
        self.array[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let array = Array::<u8, 3>::from_array([0x00; 3]);
        unsafe {
            let a0 = array.get_from_index(0);
            let a1 = array.get_from_index(1);
            let a2 = array.get_from_index(2);
            assert_eq!(0x00, a0);
            assert_eq!(0x00, a1);
            assert_eq!(0x00, a2);
        }
    }

    #[test]
    fn it_reads_and_write() {
        let mut array = Array::from_array([0x00; 3]);
        unsafe {
            array.put_to_index(2, 0x02);
            array.put_to_index(1, 0x01);
            let a0 = array.get_from_index(0);
            let a1 = array.get_from_index(1);
            let a2 = array.get_from_index(2);
            assert_eq!(0x00, a0);
            assert_eq!(0x01, a1);
            assert_eq!(0x02, a2);
        }
    }
}
