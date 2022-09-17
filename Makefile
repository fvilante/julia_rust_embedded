# I'm using an old version of avr-size (v2.26 (2015)) because the updated version is not as better in my opinion. (some useful parameters seem to being removed)
AVR_SIZE=C:\avr\avr8-gnu-toolchain-win32_x86/bin/avr-size
MCU=atmega328p
BUILD_DIR=target/avr-atmega328p/release

all: fast

# watch executes the tests if any file on the project change (note: ignore files in target and .git folder)
# if you do not have cargo-watch instaled type "cargo install cargo-watch" to install from source.
watch: 
	cargo watch -c --why -s "make test"

fast: build upload size

full: check test build upload size

check:
	cargo check --package avr_main -Z build-std=core --target .\avr_main\avr-specs\avr-atmega328p.json --release 

test: 
	cargo test --package lib_1 --release

build:
	cargo build --package avr_main -Z build-std=core --target .\avr_main\avr-specs\avr-atmega328p.json --release 

upload:
	#avrdude -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/avr_main:e
	avrdude  -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/avr_main.elf -U lfuse:w:0xFF:m -U hfuse:w:0xDE:m -U efuse:w:0xFD:m  

size:
	@$(AVR_SIZE) --format=sysv $(BUILD_DIR)/avr_main.elf
	@$(AVR_SIZE) --mcu=$(MCU) $(BUILD_DIR)/avr_main.elf
	@$(AVR_SIZE) -C --mcu=$(MCU) $(BUILD_DIR)/avr_main.elf
	