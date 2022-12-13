#TODO: To build no_std and std in some project see https://stackoverflow.com/questions/69378710/how-to-write-a-crate-so-that-std-and-no-std-can-coexist-in-different-modules


# I'm using an old version of avr-size (v2.26 (2015)) because the updated version is not as better in my opinion. (some useful parameters seem to being removed)
AVR_SIZE=C:\avr\avr8-gnu-toolchain-win32_x86/bin/avr-size
MCU=atmega328p
BUILD_DIR=target/avr-atmega328p/release

all: fast

# just a division marker
div: format
	@echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
	@echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
	@echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"

# watch platform-dependent (avr-side) and platform-independent (x86-side)
watch_all:
	cargo watch -c --why -s "make div x86_test check"

format:
	cargo fmt

# watch executes the tests if any file on the project change (note: ignore files in target and .git folder)
# if you do not have cargo-watch instaled type "cargo install cargo-watch" to install from source.
watch_x86: 
	cargo watch -c --why -s "make div x86_test"

watch_avr: 
	cargo watch -c --why -s "make div check"

watch_avr_upload:
	cargo watch -c --why -s "make div fast"


fast: build upload_fast size

full: check test build upload_fast size

check:
	cargo check --package avr_main -Z build-std=core,alloc --target .\avr_main\avr-specs\avr-atmega328p.json --release 


# tests in the platform agnostic lib are performed in x86 host
x86_test: 
	cargo test --package lib_1 --release

build:
	cargo build --package avr_main -Z build-std=core,alloc --target .\avr_main\avr-specs\avr-atmega328p.json --release 

# slow because it verifies after write proccess
upload_slow:
	#avrdude -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/avr_main:e
	avrdude  -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/avr_main.elf -U lfuse:w:0xFF:m -U hfuse:w:0xDE:m -U efuse:w:0xFD:m  

# do not verify after write
upload_fast:
	avrdude  -V -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/avr_main.elf -U lfuse:w:0xFF:m -U hfuse:w:0xDE:m -U efuse:w:0xFD:m  


size:
	@$(AVR_SIZE) --format=sysv $(BUILD_DIR)/avr_main.elf
	@$(AVR_SIZE) --mcu=$(MCU) $(BUILD_DIR)/avr_main.elf
	@$(AVR_SIZE) -C --mcu=$(MCU) $(BUILD_DIR)/avr_main.elf
	