AVR_SIZE=avr-size # I'm using an old version of avr size because the updated seems to not be working at first glance.
MCU=atmega328p
BUILD_DIR=target/avr-atmega328p/release

all: build upload show_size

build:
	cargo build --target avr-atmega328p.json -Z build-std=core --all --release

upload:
	#avrdude -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/Julia_AVR_Rust:e
	avrdude  -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/Julia_AVR_Rust.elf -U lfuse:w:0xFF:m -U hfuse:w:0xDE:m -U efuse:w:0xFD:m  

show_size:
	@$(AVR_SIZE) -C --mcu=$(MCU) $(BUILD_DIR)/Julia_AVR_Rust.elf
	@$(AVR_SIZE) --mcu=$(MCU) $(BUILD_DIR)/Julia_AVR_Rust.elf