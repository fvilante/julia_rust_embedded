# I'm using an old version of avr-size (v2.26 (2015)) because the updated version is not as better in my opinion. (some useful parameters seem to being removed)
AVR_SIZE=C:\avr\avr8-gnu-toolchain-win32_x86/bin/avr-size
MCU=atmega328p
BUILD_DIR=target/avr-atmega328p/release

all: build upload size

build:
	cargo build -Z build-std=core --target .\avr_main\avr-specs\avr-atmega328p.json --release

upload:
	#avrdude -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/Julia_AVR_Rust:e
	avrdude  -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/Julia_AVR_Rust.elf -U lfuse:w:0xFF:m -U hfuse:w:0xDE:m -U efuse:w:0xFD:m  

size:
	@$(AVR_SIZE) --format=sysv $(BUILD_DIR)/Julia_AVR_Rust.elf
	@$(AVR_SIZE) --mcu=$(MCU) $(BUILD_DIR)/Julia_AVR_Rust.elf
	@$(AVR_SIZE) -C --mcu=$(MCU) $(BUILD_DIR)/Julia_AVR_Rust.elf
	