
./target/avr-atmega328p/release/bare_metal.elf:     file format elf32-avr


Disassembly of section .text:

00000000 <__vectors>:
   0:	0c 94 34 00 	jmp	0x68	; 0x68 <__ctors_end>
   4:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
   8:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
   c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  10:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  14:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  18:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  1c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  20:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  24:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  28:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  2c:	0c 94 a0 00 	jmp	0x140	; 0x140 <__vector_11>
  30:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  34:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  38:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  3c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  40:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  44:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  48:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  4c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  50:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  54:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  58:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  5c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  60:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  64:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>

00000068 <__ctors_end>:
  68:	11 24       	eor	r1, r1
  6a:	1f be       	out	0x3f, r1	; 63
  6c:	cf ef       	ldi	r28, 0xFF	; 255
  6e:	d8 e0       	ldi	r29, 0x08	; 8
  70:	de bf       	out	0x3e, r29	; 62
  72:	cd bf       	out	0x3d, r28	; 61

00000074 <__do_copy_data>:
  74:	11 e0       	ldi	r17, 0x01	; 1
  76:	a0 e0       	ldi	r26, 0x00	; 0
  78:	b1 e0       	ldi	r27, 0x01	; 1
  7a:	e4 e0       	ldi	r30, 0x04	; 4
  7c:	f5 e0       	ldi	r31, 0x05	; 5
  7e:	02 c0       	rjmp	.+4      	; 0x84 <__do_copy_data+0x10>
  80:	05 90       	lpm	r0, Z+
  82:	0d 92       	st	X+, r0
  84:	ac 32       	cpi	r26, 0x2C	; 44
  86:	b1 07       	cpc	r27, r17
  88:	d9 f7       	brne	.-10     	; 0x80 <__do_copy_data+0xc>

0000008a <__do_clear_bss>:
  8a:	21 e0       	ldi	r18, 0x01	; 1
  8c:	ac e2       	ldi	r26, 0x2C	; 44
  8e:	b1 e0       	ldi	r27, 0x01	; 1
  90:	01 c0       	rjmp	.+2      	; 0x94 <.do_clear_bss_start>

00000092 <.do_clear_bss_loop>:
  92:	1d 92       	st	X+, r1

00000094 <.do_clear_bss_start>:
  94:	a5 33       	cpi	r26, 0x35	; 53
  96:	b2 07       	cpc	r27, r18
  98:	e1 f7       	brne	.-8      	; 0x92 <.do_clear_bss_loop>
  9a:	0e 94 e2 00 	call	0x1c4	; 0x1c4 <main>
  9e:	0c 94 80 02 	jmp	0x500	; 0x500 <_exit>

000000a2 <__bad_interrupt>:
  a2:	0c 94 00 00 	jmp	0	; 0x0 <__vectors>

000000a6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>:
  a6:	98 b1       	in	r25, 0x08	; 8
        port::C2::set_low();
    };
}

fn lcd_db4(value: bool) {
    if value == HIGH {
  a8:	97 7f       	andi	r25, 0xF7	; 247
  aa:	28 2f       	mov	r18, r24
  ac:	22 0f       	add	r18, r18
  ae:	32 2f       	mov	r19, r18
  b0:	33 0f       	add	r19, r19
  b2:	33 0f       	add	r19, r19
  b4:	38 70       	andi	r19, 0x08	; 8
  b6:	39 2b       	or	r19, r25
  b8:	38 b9       	out	0x08, r19	; 8
    if value == HIGH {
  ba:	24 70       	andi	r18, 0x04	; 4
  bc:	98 b1       	in	r25, 0x08	; 8
  be:	9b 7f       	andi	r25, 0xFB	; 251
  c0:	92 2b       	or	r25, r18
  c2:	98 b9       	out	0x08, r25	; 8
    if value == HIGH {
  c4:	86 95       	lsr	r24
  c6:	98 2f       	mov	r25, r24
  c8:	92 70       	andi	r25, 0x02	; 2
  ca:	28 b1       	in	r18, 0x08	; 8
  cc:	2d 7f       	andi	r18, 0xFD	; 253
  ce:	29 2b       	or	r18, r25
  d0:	28 b9       	out	0x08, r18	; 8
    if value == HIGH {
  d2:	86 95       	lsr	r24
  d4:	86 95       	lsr	r24
  d6:	81 70       	andi	r24, 0x01	; 1
  d8:	98 b1       	in	r25, 0x08	; 8
  da:	9e 7f       	andi	r25, 0xFE	; 254
  dc:	98 2b       	or	r25, r24
  de:	98 b9       	out	0x08, r25	; 8
  e0:	2d 98       	cbi	0x05, 5	; 5
  e2:	85 e0       	ldi	r24, 0x05	; 5
  e4:	90 e0       	ldi	r25, 0x00	; 0
                 i = inout(reg_iw) zero => _,
            )
        }
    }
    unsafe {
        asm!("1: sbiw {i}, 1",
  e6:	fc 01       	movw	r30, r24
  e8:	31 97       	sbiw	r30, 0x01	; 1
  ea:	f1 f7       	brne	.-4      	; 0xe8 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE+0x42>
  ec:	2d 9a       	sbi	0x05, 5	; 5
  ee:	01 97       	sbiw	r24, 0x01	; 1
  f0:	f1 f7       	brne	.-4      	; 0xee <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE+0x48>
  f2:	2d 98       	cbi	0x05, 5	; 5
  f4:	81 e9       	ldi	r24, 0x91	; 145
  f6:	91 e0       	ldi	r25, 0x01	; 1
  f8:	01 97       	sbiw	r24, 0x01	; 1
  fa:	f1 f7       	brne	.-4      	; 0xf8 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE+0x52>
    lcd_db5(get_bit_at_as_bool(data, 1));
    lcd_db6(get_bit_at_as_bool(data, 2));
    lcd_db7(get_bit_at_as_bool(data, 3));
    // pulse enable
    pulse_enable();
}
  fc:	08 95       	ret

000000fe <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E>:
pub fn print_char(char: char) {
    write_u8(char as u8);
}

/// prints a full string
pub fn print(text: &str) {
  fe:	cf 92       	push	r12
 100:	df 92       	push	r13
 102:	ff 92       	push	r15
 104:	0f 93       	push	r16
 106:	1f 93       	push	r17
 108:	8b 01       	movw	r16, r22
 10a:	dc 01       	movw	r26, r24
                unsafe {
                    assume(!self.ptr.as_ptr().is_null());
                    if !<T>::IS_ZST {
                        assume(!self.end.is_null());
                    }
                    if is_empty!(self) {
 10c:	00 30       	cpi	r16, 0x00	; 0
 10e:	11 05       	cpc	r17, r1
 110:	89 f0       	breq	.+34     	; 0x134 <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E+0x36>
 112:	0a 0f       	add	r16, r26
 114:	1b 1f       	adc	r17, r27
    for char in text.as_bytes() {
        write_u8(*char);
 116:	fd 90       	ld	r15, X+
 118:	2c 9a       	sbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 11a:	8f 2d       	mov	r24, r15
 11c:	82 95       	swap	r24
 11e:	8f 70       	andi	r24, 0x0F	; 15
 120:	6d 01       	movw	r12, r26
 122:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 126:	8f 2d       	mov	r24, r15
 128:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 12c:	d6 01       	movw	r26, r12
 12e:	a0 17       	cp	r26, r16
 130:	b1 07       	cpc	r27, r17
 132:	89 f7       	brne	.-30     	; 0x116 <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E+0x18>
    }
}
 134:	1f 91       	pop	r17
 136:	0f 91       	pop	r16
 138:	ff 90       	pop	r15
 13a:	df 90       	pop	r13
 13c:	cf 90       	pop	r12
 13e:	08 95       	ret

00000140 <__vector_11>:
    configure_timer();
    enable_interrupts_globally();
}

// This is the timer interruption handler
#[avr_device::interrupt(atmega328p)]
 140:	0f 92       	push	r0
 142:	1f 92       	push	r1
 144:	0f b6       	in	r0, 0x3f	; 63
 146:	0f 92       	push	r0
 148:	11 24       	eor	r1, r1
 14a:	2f 93       	push	r18
 14c:	3f 93       	push	r19
 14e:	4f 93       	push	r20
 150:	5f 93       	push	r21
 152:	6f 93       	push	r22
 154:	7f 93       	push	r23
 156:	8f 93       	push	r24
 158:	9f 93       	push	r25
    fn new() -> Self {
        Self { count: 0x00 }
    }

    fn increment(&mut self) {
        self.count += 1;
 15a:	80 91 2c 01 	lds	r24, 0x012C	; 0x80012c <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0>
 15e:	90 91 2d 01 	lds	r25, 0x012D	; 0x80012d <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x1>
 162:	20 91 2e 01 	lds	r18, 0x012E	; 0x80012e <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x2>
 166:	30 91 2f 01 	lds	r19, 0x012F	; 0x80012f <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x3>
 16a:	40 91 30 01 	lds	r20, 0x0130	; 0x800130 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x4>
 16e:	50 91 31 01 	lds	r21, 0x0131	; 0x800131 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x5>
 172:	60 91 32 01 	lds	r22, 0x0132	; 0x800132 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x6>
 176:	70 91 33 01 	lds	r23, 0x0133	; 0x800133 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x7>
 17a:	8f 5f       	subi	r24, 0xFF	; 255
 17c:	9f 4f       	sbci	r25, 0xFF	; 255
 17e:	2f 4f       	sbci	r18, 0xFF	; 255
 180:	3f 4f       	sbci	r19, 0xFF	; 255
 182:	4f 4f       	sbci	r20, 0xFF	; 255
 184:	5f 4f       	sbci	r21, 0xFF	; 255
 186:	6f 4f       	sbci	r22, 0xFF	; 255
 188:	7f 4f       	sbci	r23, 0xFF	; 255
 18a:	50 93 31 01 	sts	0x0131, r21	; 0x800131 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x5>
 18e:	40 93 30 01 	sts	0x0130, r20	; 0x800130 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x4>
 192:	70 93 33 01 	sts	0x0133, r23	; 0x800133 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x7>
 196:	60 93 32 01 	sts	0x0132, r22	; 0x800132 <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x6>
 19a:	90 93 2d 01 	sts	0x012D, r25	; 0x80012d <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x1>
 19e:	80 93 2c 01 	sts	0x012C, r24	; 0x80012c <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0>
 1a2:	30 93 2f 01 	sts	0x012F, r19	; 0x80012f <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x3>
 1a6:	20 93 2e 01 	sts	0x012E, r18	; 0x80012e <_ZN10bare_metal14microcontroler5timer15INTERRUPT_STATE17hb67ecbd5f498814dE.0+0x2>
#[avr_device::interrupt(atmega328p)]
 1aa:	9f 91       	pop	r25
 1ac:	8f 91       	pop	r24
 1ae:	7f 91       	pop	r23
 1b0:	6f 91       	pop	r22
 1b2:	5f 91       	pop	r21
 1b4:	4f 91       	pop	r20
 1b6:	3f 91       	pop	r19
 1b8:	2f 91       	pop	r18
 1ba:	0f 90       	pop	r0
 1bc:	0f be       	out	0x3f, r0	; 63
 1be:	1f 90       	pop	r1
 1c0:	0f 90       	pop	r0
 1c2:	18 95       	reti

000001c4 <main>:
use microcontroler::timer::init_timer;
//use panic_halt as _;

// Initialize app
#[no_mangle] // TODO: it seems this line is not necessary remove it if possible.
#[arduino_hal::entry]
 1c4:	0e 94 e6 00 	call	0x1cc	; 0x1cc <__avr_device_rt_main>
 1c8:	0e 94 6b 02 	call	0x4d6	; 0x4d6 <abort>

000001cc <__avr_device_rt_main>:
fn main() -> ! {
 1cc:	8f 92       	push	r8
 1ce:	9f 92       	push	r9
 1d0:	af 92       	push	r10
 1d2:	bf 92       	push	r11
 1d4:	cf 92       	push	r12
 1d6:	df 92       	push	r13
 1d8:	ef 92       	push	r14
 1da:	ff 92       	push	r15
 1dc:	0f 93       	push	r16
 1de:	1f 93       	push	r17
 1e0:	cf 93       	push	r28
 1e2:	df 93       	push	r29
 1e4:	cd b7       	in	r28, 0x3d	; 61
 1e6:	de b7       	in	r29, 0x3e	; 62
    unsafe {
        assert_unsafe_precondition!(
            "ptr::write_volatile requires that the pointer argument is aligned and non-null",
            [T](dst: *mut T) => is_aligned_and_not_null(dst)
        );
        intrinsics::volatile_store(dst, src);
 1e8:	c4 50       	subi	r28, 0x04	; 4
 1ea:	d8 40       	sbci	r29, 0x08	; 8
 1ec:	0f b6       	in	r0, 0x3f	; 63
 1ee:	f8 94       	cli
 1f0:	de bf       	out	0x3e, r29	; 62
 1f2:	0f be       	out	0x3f, r0	; 63
 1f4:	cd bf       	out	0x3d, r28	; 61
 1f6:	24 9a       	sbi	0x04, 4	; 4
 1f8:	25 9a       	sbi	0x04, 5	; 4
 1fa:	38 9a       	sbi	0x07, 0	; 7
 1fc:	39 9a       	sbi	0x07, 1	; 7
 1fe:	3a 9a       	sbi	0x07, 2	; 7
 200:	3b 9a       	sbi	0x07, 3	; 7
 202:	80 e0       	ldi	r24, 0x00	; 0
 204:	90 e0       	ldi	r25, 0x00	; 0
 206:	9c 01       	movw	r18, r24
 208:	ac 01       	movw	r20, r24
 20a:	bc 01       	movw	r22, r24
 20c:	fc 01       	movw	r30, r24
            asm!("1: sbiw {i}, 1",
 20e:	dc 01       	movw	r26, r24
 210:	11 97       	sbiw	r26, 0x01	; 1
 212:	f1 f7       	brne	.-4      	; 0x210 <__avr_device_rt_main+0x44>
 214:	a1 e0       	ldi	r26, 0x01	; 1
                #[inline]
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
                #[inline]
                fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
 216:	22 30       	cpi	r18, 0x02	; 2
 218:	31 05       	cpc	r19, r1
 21a:	48 07       	cpc	r20, r24
 21c:	59 07       	cpc	r21, r25
 21e:	68 07       	cpc	r22, r24
 220:	79 07       	cpc	r23, r25
 222:	e8 07       	cpc	r30, r24
 224:	f9 07       	cpc	r31, r25
 226:	08 f0       	brcs	.+2      	; 0x22a <__avr_device_rt_main+0x5e>
 228:	a1 2d       	mov	r26, r1
 22a:	a1 70       	andi	r26, 0x01	; 1
        #[inline(always)]
        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
        pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
            // SAFETY: the caller must uphold the safety contract for
            // `unchecked_add`.
            unsafe { intrinsics::unchecked_add(self, rhs) }
 22c:	2f 5f       	subi	r18, 0xFF	; 255
 22e:	3f 4f       	sbci	r19, 0xFF	; 255
 230:	4f 4f       	sbci	r20, 0xFF	; 255
 232:	5f 4f       	sbci	r21, 0xFF	; 255
 234:	6f 4f       	sbci	r22, 0xFF	; 255
 236:	7f 4f       	sbci	r23, 0xFF	; 255
 238:	ef 4f       	sbci	r30, 0xFF	; 255
 23a:	ff 4f       	sbci	r31, 0xFF	; 255
}

impl<T: TrustedStep> RangeIteratorImpl for ops::Range<T> {
    #[inline]
    fn spec_next(&mut self) -> Option<T> {
        if self.start < self.end {
 23c:	a0 30       	cpi	r26, 0x00	; 0
 23e:	39 f7       	brne	.-50     	; 0x20e <__avr_device_rt_main+0x42>
 240:	81 e4       	ldi	r24, 0x41	; 65
 242:	9d e0       	ldi	r25, 0x0D	; 13
        asm!("1: sbiw {i}, 1",
 244:	01 97       	sbiw	r24, 0x01	; 1
 246:	f1 f7       	brne	.-4      	; 0x244 <__avr_device_rt_main+0x78>
 248:	2c 98       	cbi	0x05, 4	; 5
 24a:	2d 98       	cbi	0x05, 5	; 5
    // put the LCD into 4 bit (or 8 bit mode, but not is the case)
    // this is according to the hitachi HD44780 datasheet
    // figure 24, pg 46

    // we start in 8bit mode, try to set 4 bit mode
    write4bits(0x03);
 24c:	83 e0       	ldi	r24, 0x03	; 3
 24e:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 252:	21 e5       	ldi	r18, 0x51	; 81
 254:	36 e4       	ldi	r19, 0x46	; 70
 256:	c9 01       	movw	r24, r18
 258:	89 01       	movw	r16, r18
 25a:	01 97       	sbiw	r24, 0x01	; 1
 25c:	f1 f7       	brne	.-4      	; 0x25a <__avr_device_rt_main+0x8e>
    delay_us(4500); // wait min 4.1ms

    // second try
    write4bits(0x03);
 25e:	83 e0       	ldi	r24, 0x03	; 3
 260:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 264:	c8 01       	movw	r24, r16
 266:	01 97       	sbiw	r24, 0x01	; 1
 268:	f1 f7       	brne	.-4      	; 0x266 <__avr_device_rt_main+0x9a>
    delay_us(4500); // wait min 4.1ms

    // third go!
    write4bits(0x03);
 26a:	83 e0       	ldi	r24, 0x03	; 3
 26c:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 270:	89 e5       	ldi	r24, 0x59	; 89
 272:	92 e0       	ldi	r25, 0x02	; 2
 274:	01 97       	sbiw	r24, 0x01	; 1
 276:	f1 f7       	brne	.-4      	; 0x274 <__avr_device_rt_main+0xa8>
    delay_us(150);

    // finally, set to 4-bit interface
    write4bits(0x02);
 278:	82 e0       	ldi	r24, 0x02	; 2
 27a:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 27e:	2c 98       	cbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 280:	82 e0       	ldi	r24, 0x02	; 2
 282:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 286:	88 e2       	ldi	r24, 0x28	; 40
 288:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 28c:	2c 98       	cbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 28e:	81 2d       	mov	r24, r1
 290:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 294:	8c e0       	ldi	r24, 0x0C	; 12
 296:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 29a:	2c 98       	cbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 29c:	81 2d       	mov	r24, r1
 29e:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 2a2:	86 e0       	ldi	r24, 0x06	; 6
 2a4:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 2a8:	2c 98       	cbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 2aa:	81 2d       	mov	r24, r1
 2ac:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 2b0:	11 e0       	ldi	r17, 0x01	; 1
    write4bits(value); // least significant bits
 2b2:	81 e0       	ldi	r24, 0x01	; 1
 2b4:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 2b8:	81 e4       	ldi	r24, 0x41	; 65
 2ba:	9f e1       	ldi	r25, 0x1F	; 31
 2bc:	01 97       	sbiw	r24, 0x01	; 1
 2be:	f1 f7       	brne	.-4      	; 0x2bc <__avr_device_rt_main+0xf0>

    lcd::lcd_initialize();
    // Initialize timer couting (1khz)
    //init_timer();

    lcd::print("oi");
 2c0:	80 e0       	ldi	r24, 0x00	; 0
 2c2:	91 e0       	ldi	r25, 0x01	; 1
 2c4:	62 e0       	ldi	r22, 0x02	; 2
 2c6:	70 e0       	ldi	r23, 0x00	; 0
 2c8:	0e 94 7f 00 	call	0xfe	; 0xfe <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E>
    let sreg;
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Store current state
            unsafe {
                asm!(
 2cc:	8f b7       	in	r24, 0x3f	; 63
            unsafe { asm!("cli") };
 2ce:	f8 94       	cli
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
 2d0:	90 91 34 01 	lds	r25, 0x0134	; 0x800134 <DEVICE_PERIPHERALS>
 2d4:	90 30       	cpi	r25, 0x00	; 0
 2d6:	09 f0       	breq	.+2      	; 0x2da <__avr_device_rt_main+0x10e>
 2d8:	f4 c0       	rjmp	.+488    	; 0x4c2 <__LOCK_REGION_LENGTH__+0xc2>
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
 2da:	10 93 34 01 	sts	0x0134, r17	; 0x800134 <DEVICE_PERIPHERALS>
pub unsafe fn restore(irq_flag: IrqFlag) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Restore global interrupt flag in SREG.
            // This also clobbers all other bits in SREG.
            asm!(
 2de:	8f bf       	out	0x3f, r24	; 63
 2e0:	2c 9a       	sbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 2e2:	83 e0       	ldi	r24, 0x03	; 3
 2e4:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 2e8:	80 e3       	ldi	r24, 0x30	; 48
 2ea:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 2ee:	2c 9a       	sbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 2f0:	83 e0       	ldi	r24, 0x03	; 3
 2f2:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 2f6:	84 e3       	ldi	r24, 0x34	; 52
 2f8:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 2fc:	2c 9a       	sbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 2fe:	83 e0       	ldi	r24, 0x03	; 3
 300:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 304:	80 e3       	ldi	r24, 0x30	; 48
 306:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 30a:	2c 9a       	sbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 30c:	83 e0       	ldi	r24, 0x03	; 3
 30e:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 312:	80 e3       	ldi	r24, 0x30	; 48
 314:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 318:	2c 9a       	sbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 31a:	86 e0       	ldi	r24, 0x06	; 6
 31c:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 320:	88 e6       	ldi	r24, 0x68	; 104
 322:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 326:	8e 01       	movw	r16, r28
 328:	0f 5f       	subi	r16, 0xFF	; 255
 32a:	1f 4f       	sbci	r17, 0xFF	; 255
 32c:	40 e0       	ldi	r20, 0x00	; 0
 32e:	54 e0       	ldi	r21, 0x04	; 4
    let mut ep = arduino_hal::Eeprom::new(dp.EEPROM);
    let ep_capacity = ep.capacity();
    //ufmt::uwriteln!(&mut serial, "eeprom capacity is:{}\r", ep_capacity).void_unwrap();
    lcd::print_u16_in_hex(ep_capacity);

    let mut data = [0_u8; arduino_hal::Eeprom::CAPACITY as usize]; //17
 330:	c8 01       	movw	r24, r16
 332:	61 2d       	mov	r22, r1
 334:	6a 01       	movw	r12, r20
 336:	0e 94 79 02 	call	0x4f2	; 0x4f2 <memset>
 33a:	80 e0       	ldi	r24, 0x00	; 0
 33c:	90 e0       	ldi	r25, 0x00	; 0
 33e:	24 e0       	ldi	r18, 0x04	; 4
 340:	41 e0       	ldi	r20, 0x01	; 1
 342:	d8 01       	movw	r26, r16
 344:	a8 0f       	add	r26, r24
 346:	b9 1f       	adc	r27, r25
        intrinsics::volatile_load(src)
 348:	3f b3       	in	r19, 0x1f	; 31
        EERE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM Write Enable"]
    #[inline(always)]
    pub fn eepe(&self) -> EEPE_R {
        EEPE_R::new(((self.bits >> 1) & 1) != 0)
 34a:	32 70       	andi	r19, 0x02	; 2
    ) => {
        mod atmega_helper {
            #[inline]
            pub unsafe fn wait_read(regs: &$EEPROM) {
                //Wait for completion of previous write.
                while regs.eecr.read().eepe().bit_is_set() {}
 34c:	30 30       	cpi	r19, 0x00	; 0
 34e:	e1 f7       	brne	.-8      	; 0x348 <__avr_device_rt_main+0x17c>
        intrinsics::volatile_store(dst, src);
 350:	92 bd       	out	0x22, r25	; 34
 352:	81 bd       	out	0x21, r24	; 33
 354:	4f bb       	out	0x1f, r20	; 31
        intrinsics::volatile_load(src)
 356:	30 b5       	in	r19, 0x20	; 32
            *byte = self.p.raw_read_byte(offset + i as u16);
 358:	3c 93       	st	X, r19
    #[inline]
    #[rustc_inherit_overflow_checks]
    fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count += 1;
 35a:	01 96       	adiw	r24, 0x01	; 1
 35c:	80 30       	cpi	r24, 0x00	; 0
 35e:	92 07       	cpc	r25, r18
 360:	81 f7       	brne	.-32     	; 0x342 <__avr_device_rt_main+0x176>
    if ep.read(0, &mut data).is_err() {
        //ufmt::uwriteln!(&mut serial, "read eeprom fail:\r").void_unwrap();
        lcd::print("read eeprom fail");
        loop {}
    } else {
        lcd::print("successful read!");
 362:	82 e0       	ldi	r24, 0x02	; 2
 364:	91 e0       	ldi	r25, 0x01	; 1
 366:	60 e1       	ldi	r22, 0x10	; 16
 368:	70 e0       	ldi	r23, 0x00	; 0
 36a:	0e 94 7f 00 	call	0xfe	; 0xfe <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E>
 36e:	8e 01       	movw	r16, r28
 370:	0f 5f       	subi	r16, 0xFF	; 255
 372:	1b 4f       	sbci	r17, 0xFB	; 251
 374:	be 01       	movw	r22, r28
 376:	6f 5f       	subi	r22, 0xFF	; 255
 378:	7f 4f       	sbci	r23, 0xFF	; 255
    }

    ////ufmt::uwriteln!(&mut serial, "Got:\r").void_unwrap();
    for i in data {
 37a:	c8 01       	movw	r24, r16
 37c:	7b 01       	movw	r14, r22
 37e:	a6 01       	movw	r20, r12
 380:	0e 94 70 02 	call	0x4e0	; 0x4e0 <memcpy>
 384:	0f b6       	in	r0, 0x3f	; 63
 386:	cb 53       	subi	r28, 0x3B	; 59
 388:	d8 4f       	sbci	r29, 0xF8	; 248
 38a:	ce ae       	std	Y+62, r12	; 0x3e
 38c:	df ae       	std	Y+63, r13	; 0x3f
 38e:	c5 5c       	subi	r28, 0xC5	; 197
 390:	d7 40       	sbci	r29, 0x07	; 7
 392:	0f be       	out	0x3f, r0	; 63
 394:	68 01       	movw	r12, r16
 396:	00 50       	subi	r16, 0x00	; 0
 398:	1c 4f       	sbci	r17, 0xFC	; 252
 39a:	58 01       	movw	r10, r16
 39c:	80 e0       	ldi	r24, 0x00	; 0
 39e:	90 e0       	ldi	r25, 0x00	; 0
 3a0:	d6 01       	movw	r26, r12
    pub const unsafe fn offset(self, count: isize) -> *const T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `offset`.
        unsafe { intrinsics::offset(self, count) }
 3a2:	a8 0f       	add	r26, r24
 3a4:	b9 1f       	adc	r27, r25
    unsafe fn next_unchecked(&mut self) -> usize {
        debug_assert!(self.start < self.end);

        let value = self.start;
        // SAFETY: The range isn't empty, so this cannot overflow
        self.start = unsafe { unchecked_add(value, 1) };
 3a6:	01 96       	adiw	r24, 0x01	; 1
 3a8:	f5 01       	movw	r30, r10
 3aa:	4c 01       	movw	r8, r24
 3ac:	80 83       	st	Z, r24
 3ae:	91 83       	std	Z+1, r25	; 0x01
        copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
 3b0:	1c 91       	ld	r17, X
use core::{marker::Destruct, str::FromStr};
use heapless::String;

/// SAFETY: Do not call this function with index outside the 0..15 range! Else this function will panic!
const fn get_char(index: u8) -> char {
    match index {
 3b2:	10 3a       	cpi	r17, 0xA0	; 160
 3b4:	80 e3       	ldi	r24, 0x30	; 48
 3b6:	08 f0       	brcs	.+2      	; 0x3ba <__avr_device_rt_main+0x1ee>
 3b8:	87 e3       	ldi	r24, 0x37	; 55
}

// NOTE: return folows the order: "Most Significant on Right"
pub const fn convert_u8_to_str_hex(data: u8) -> (char, char) {
    let low: u8 = data & 0b00001111;
    let high: u8 = (data & 0b11110000) >> 4;
 3ba:	01 2f       	mov	r16, r17
 3bc:	02 95       	swap	r16
 3be:	0f 70       	andi	r16, 0x0F	; 15
 3c0:	08 0f       	add	r16, r24
    write4bits(value >> 4); // most significant bits
 3c2:	80 2f       	mov	r24, r16
 3c4:	82 95       	swap	r24
 3c6:	8f 70       	andi	r24, 0x0F	; 15
 3c8:	2c 9a       	sbi	0x05, 4	; 5
 3ca:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 3ce:	80 2f       	mov	r24, r16
 3d0:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    let low: u8 = data & 0b00001111;
 3d4:	1f 70       	andi	r17, 0x0F	; 15
    match index {
 3d6:	1a 30       	cpi	r17, 0x0A	; 10
 3d8:	00 e3       	ldi	r16, 0x30	; 48
 3da:	08 f0       	brcs	.+2      	; 0x3de <__avr_device_rt_main+0x212>
 3dc:	07 e3       	ldi	r16, 0x37	; 55
 3de:	01 0f       	add	r16, r17
    write4bits(value >> 4); // most significant bits
 3e0:	80 2f       	mov	r24, r16
 3e2:	82 95       	swap	r24
 3e4:	8f 70       	andi	r24, 0x0F	; 15
 3e6:	2c 9a       	sbi	0x05, 4	; 5
 3e8:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 3ec:	80 2f       	mov	r24, r16
 3ee:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 3f2:	2c 9a       	sbi	0x05, 4	; 5
    write4bits(value >> 4); // most significant bits
 3f4:	86 e0       	ldi	r24, 0x06	; 6
 3f6:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
    write4bits(value); // least significant bits
 3fa:	88 e6       	ldi	r24, 0x68	; 104
 3fc:	0e 94 53 00 	call	0xa6	; 0xa6 <_ZN10bare_metal5board3lcd6driver10write4bits17h366fae1645a40dceE>
 400:	c4 01       	movw	r24, r8
impl Iterator for IndexRange {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.len() > 0 {
 402:	80 30       	cpi	r24, 0x00	; 0
 404:	24 e0       	ldi	r18, 0x04	; 4
 406:	92 07       	cpc	r25, r18
 408:	09 f0       	breq	.+2      	; 0x40c <__LOCK_REGION_LENGTH__+0xc>
 40a:	ca cf       	rjmp	.-108    	; 0x3a0 <__avr_device_rt_main+0x1d4>
        lcd::print_u8_in_hex(i);
        //delay_ms(1000);
        //    //ufmt::uwriteln!(&mut serial, "{}", i).void_unwrap();
    }

    lcd::print("apagando...");
 40c:	82 e1       	ldi	r24, 0x12	; 18
 40e:	91 e0       	ldi	r25, 0x01	; 1
 410:	6b e0       	ldi	r22, 0x0B	; 11
 412:	70 e0       	ldi	r23, 0x00	; 0
 414:	0e 94 7f 00 	call	0xfe	; 0xfe <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E>
 418:	80 e0       	ldi	r24, 0x00	; 0
 41a:	90 e0       	ldi	r25, 0x00	; 0
 41c:	24 e1       	ldi	r18, 0x14	; 20
 41e:	32 e0       	ldi	r19, 0x02	; 2
 420:	43 e0       	ldi	r20, 0x03	; 3
 422:	87 01       	movw	r16, r14
        intrinsics::volatile_load(src)
 424:	5f b3       	in	r21, 0x1f	; 31
 426:	52 70       	andi	r21, 0x02	; 2
                while regs.eecr.read().eepe().bit_is_set() {}
 428:	50 30       	cpi	r21, 0x00	; 0
 42a:	e1 f7       	brne	.-8      	; 0x424 <__LOCK_REGION_LENGTH__+0x24>
        intrinsics::volatile_store(dst, src);
 42c:	92 bd       	out	0x22, r25	; 34
 42e:	81 bd       	out	0x21, r24	; 33
 430:	2f bb       	out	0x1f, r18	; 31
 432:	3f bb       	out	0x1f, r19	; 31
 434:	fc 01       	movw	r30, r24
 436:	31 96       	adiw	r30, 0x01	; 1
 438:	8f 3f       	cpi	r24, 0xFF	; 255
 43a:	94 07       	cpc	r25, r20
 43c:	cf 01       	movw	r24, r30
 43e:	90 f3       	brcs	.-28     	; 0x424 <__LOCK_REGION_LENGTH__+0x24>
    let _ = ep.erase(0, arduino_hal::Eeprom::CAPACITY);
    lcd::print("fim");
 440:	8d e1       	ldi	r24, 0x1D	; 29
 442:	91 e0       	ldi	r25, 0x01	; 1
 444:	63 e0       	ldi	r22, 0x03	; 3
 446:	70 e0       	ldi	r23, 0x00	; 0
 448:	0e 94 7f 00 	call	0xfe	; 0xfe <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E>
 44c:	40 e0       	ldi	r20, 0x00	; 0
 44e:	54 e0       	ldi	r21, 0x04	; 4
}

impl<T: Copy> SpecFill<T> for [T] {
    fn spec_fill(&mut self, value: T) {
        for item in self.iter_mut() {
            *item = value;
 450:	c8 01       	movw	r24, r16
 452:	67 e7       	ldi	r22, 0x77	; 119
 454:	0e 94 79 02 	call	0x4f2	; 0x4f2 <memset>

    data.fill(0x77);

    lcd::print("gravando...");
 458:	80 e2       	ldi	r24, 0x20	; 32
 45a:	91 e0       	ldi	r25, 0x01	; 1
 45c:	6b e0       	ldi	r22, 0x0B	; 11
 45e:	70 e0       	ldi	r23, 0x00	; 0
 460:	0e 94 7f 00 	call	0xfe	; 0xfe <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E>
 464:	80 e0       	ldi	r24, 0x00	; 0
 466:	90 e0       	ldi	r25, 0x00	; 0
 468:	21 e0       	ldi	r18, 0x01	; 1
 46a:	34 e0       	ldi	r19, 0x04	; 4
 46c:	42 e0       	ldi	r20, 0x02	; 2
 46e:	fc 01       	movw	r30, r24
 470:	d8 01       	movw	r26, r16
 472:	ae 0f       	add	r26, r30
 474:	bf 1f       	adc	r27, r31
            self.p.raw_write_byte(offset + i as u16, *byte)
 476:	5c 91       	ld	r21, X
 478:	01 96       	adiw	r24, 0x01	; 1
        intrinsics::volatile_load(src)
 47a:	6f b3       	in	r22, 0x1f	; 31
 47c:	62 70       	andi	r22, 0x02	; 2
                while regs.eecr.read().eepe().bit_is_set() {}
 47e:	60 30       	cpi	r22, 0x00	; 0
 480:	e1 f7       	brne	.-8      	; 0x47a <__LOCK_REGION_LENGTH__+0x7a>
        intrinsics::volatile_store(dst, src);
 482:	f2 bd       	out	0x22, r31	; 34
 484:	e1 bd       	out	0x21, r30	; 33
 486:	2f bb       	out	0x1f, r18	; 31
        intrinsics::volatile_load(src)
 488:	60 b5       	in	r22, 0x20	; 32
                    let diff_mask = old_value ^ data;
 48a:	65 27       	eor	r22, r21
                    if (diff_mask & data) != 0 {
 48c:	76 2f       	mov	r23, r22
 48e:	75 23       	and	r23, r21
 490:	70 30       	cpi	r23, 0x00	; 0
 492:	21 f0       	breq	.+8      	; 0x49c <__LOCK_REGION_LENGTH__+0x9c>
                        if data != 0xff {
 494:	5f 3f       	cpi	r21, 0xFF	; 255
 496:	39 f0       	breq	.+14     	; 0x4a6 <__LOCK_REGION_LENGTH__+0xa6>
 498:	64 e0       	ldi	r22, 0x04	; 4
 49a:	03 c0       	rjmp	.+6      	; 0x4a2 <__LOCK_REGION_LENGTH__+0xa2>
                        if diff_mask != 0 {
 49c:	60 30       	cpi	r22, 0x00	; 0
 49e:	31 f0       	breq	.+12     	; 0x4ac <__LOCK_REGION_LENGTH__+0xac>
 4a0:	64 e2       	ldi	r22, 0x24	; 36
 4a2:	50 bd       	out	0x20, r21	; 32
 4a4:	01 c0       	rjmp	.+2      	; 0x4a8 <__LOCK_REGION_LENGTH__+0xa8>
 4a6:	64 e1       	ldi	r22, 0x14	; 20
 4a8:	6f bb       	out	0x1f, r22	; 31
 4aa:	4f bb       	out	0x1f, r20	; 31
 4ac:	80 30       	cpi	r24, 0x00	; 0
 4ae:	93 07       	cpc	r25, r19
 4b0:	09 f0       	breq	.+2      	; 0x4b4 <__LOCK_REGION_LENGTH__+0xb4>
 4b2:	dd cf       	rjmp	.-70     	; 0x46e <__LOCK_REGION_LENGTH__+0x6e>
    ep.write(0, &mut data);
    lcd::print("fim");
 4b4:	8d e1       	ldi	r24, 0x1D	; 29
 4b6:	91 e0       	ldi	r25, 0x01	; 1
 4b8:	63 e0       	ldi	r22, 0x03	; 3
 4ba:	70 e0       	ldi	r23, 0x00	; 0
 4bc:	0e 94 7f 00 	call	0xfe	; 0xfe <_ZN10bare_metal5board3lcd6driver5print17hb88c45a9cb7fb034E>

    loop {}
 4c0:	ff cf       	rjmp	.-2      	; 0x4c0 <__LOCK_REGION_LENGTH__+0xc0>
 4c2:	8f bf       	out	0x3f, r24	; 63
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_option", issue = "67441")]
    pub const fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic("called `Option::unwrap()` on a `None` value"),
 4c4:	0e 94 67 02 	call	0x4ce	; 0x4ce <_ZN4core9panicking5panic17heed53d1878cc450fE>
 4c8:	0e 94 6b 02 	call	0x4d6	; 0x4d6 <abort>

000004cc <_ZN4core9panicking9panic_fmt17ha13661cdd4f23fc9E>:
/// ATTENTION: Do not call it directly, instead use the [`fatal_error!`] macro
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // IMPORTANT: Please notice that using `PanicInfo::message` consumes a lot of memory (about 15% of total Flash, and 40% of total Ram)
    // So we are not using it in the platform embedded. Insted we are using the macro [`fatal_error!`]
    loop {}
 4cc:	ff cf       	rjmp	.-2      	; 0x4cc <_ZN4core9panicking9panic_fmt17ha13661cdd4f23fc9E>

000004ce <_ZN4core9panicking5panic17heed53d1878cc450fE>:
    // reduce size overhead. The format_args! macro uses str's Display trait to
    // write expr, which calls Formatter::pad, which must accommodate string
    // truncation and padding (even though none is used here). Using
    // Arguments::new_v1 may allow the compiler to omit Formatter::pad from the
    // output binary, saving up to a few kilobytes.
    panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
 4ce:	0e 94 66 02 	call	0x4cc	; 0x4cc <_ZN4core9panicking9panic_fmt17ha13661cdd4f23fc9E>
 4d2:	0e 94 6b 02 	call	0x4d6	; 0x4d6 <abort>

000004d6 <abort>:
 4d6:	81 e0       	ldi	r24, 0x01	; 1
 4d8:	90 e0       	ldi	r25, 0x00	; 0
 4da:	f8 94       	cli
 4dc:	0c 94 80 02 	jmp	0x500	; 0x500 <_exit>

000004e0 <memcpy>:
 4e0:	fb 01       	movw	r30, r22
 4e2:	dc 01       	movw	r26, r24
 4e4:	02 c0       	rjmp	.+4      	; 0x4ea <memcpy+0xa>
 4e6:	01 90       	ld	r0, Z+
 4e8:	0d 92       	st	X+, r0
 4ea:	41 50       	subi	r20, 0x01	; 1
 4ec:	50 40       	sbci	r21, 0x00	; 0
 4ee:	d8 f7       	brcc	.-10     	; 0x4e6 <memcpy+0x6>
 4f0:	08 95       	ret

000004f2 <memset>:
 4f2:	dc 01       	movw	r26, r24
 4f4:	01 c0       	rjmp	.+2      	; 0x4f8 <memset+0x6>
 4f6:	6d 93       	st	X+, r22
 4f8:	41 50       	subi	r20, 0x01	; 1
 4fa:	50 40       	sbci	r21, 0x00	; 0
 4fc:	e0 f7       	brcc	.-8      	; 0x4f6 <memset+0x4>
 4fe:	08 95       	ret

00000500 <_exit>:
 500:	f8 94       	cli

00000502 <__stop_program>:
 502:	ff cf       	rjmp	.-2      	; 0x502 <__stop_program>
