
static mut SOME_VAR: u64 = 0x00;


pub fn delay_us(us: u64) {
    fn delay_one_us() {
        for n in 0..1000 {
            unsafe { SOME_VAR = SOME_VAR.wrapping_add(n) }
        }
    }
    for _ in 0..us {
        delay_one_us();
    }
}