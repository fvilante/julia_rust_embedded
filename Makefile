#TODO: To build no_std and std in some project see https://stackoverflow.com/questions/69378710/how-to-write-a-crate-so-that-std-and-no-std-can-coexist-in-different-modules


# I'm using an old version of avr-size (v2.26 (2015)) because the updated version is not as better in my opinion. (some useful parameters seem to being removed)
AVR_SIZE=C:\avr\avr8-gnu-toolchain-win32_x86/bin/avr-size
MCU=atmega328p
BUILD_DIR=target/avr-atmega328p/release
BASE=-Z build-std=core,alloc --target .\bare_metal\avr-specs\avr-atmega328p.json --release

all: fast

# To make cargo compiler to show verbose information when error happens (specially useful when
# runing tests), its necesssary to set enviroment variable "RUST_BACKTRACE=full". The line below
# works when typed direct into powershell.
# TODO: Make rule below to also work as a make file rule 
verbose:
	$env:RUST_BACKTRACE="full"


# just a division marker
div:
	@echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
	@echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
	@echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"

# watch platform-dependent (avr-side) and platform-independent (x86-side)
watch_all:
	cargo watch -c --why -s "make div test check"

format:
	cargo fmt

# watch executes the tests if any file on the project change (note: ignore files in target and .git folder)
# if you do not have cargo-watch instaled type "cargo install cargo-watch" to install from source.
watch_x86_test: 
	cargo watch -c --why -s "make div test"

watch_avr: 
	cargo watch -c --why -s "make div check size"

watch_avr_upload: 
	cargo watch -c --why -s "make div format fast"

watch_avr_upload_fix: fix_silently watch_avr_upload

fast: build upload_fast size

full: check test build upload_fast size

clippy:
	cargo clippy --package bare_metal $(BASE)  

check:
	cargo check --package bare_metal $(BASE) 

# Try to automatically correct as many warnings as possible. Note: Assure to have `warning` enabled in your main module. Disabled warnings are not fixed.
# NOTE: you can also use `--allow-dirty --allow-staged` parameters if you want to overwrite fixed files even if they are uncommited in the repository.
fix:
	cargo fix --package bare_metal $(BASE) --bins 

# Same as `fix` rule but do not print fix log messages
# IMPORTANT: if you have uncommited changes this rule may overwrite your files unadivertedly
fix_silently:
	cargo fix --package bare_metal $(BASE) --bins --quiet --allow-dirty --allow-staged


# tests in the platform agnostic lib are performed in x86 host
test: 
	cargo test --package cross_platform --release

build:
	cargo build --package bare_metal $(BASE) 

doc:
	cargo doc --package bare_metal $(BASE) --open

# slow because it verifies after write proccess
upload_slow:
	#avrdude -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/bare_metal:e
	avrdude  -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/bare_metal.elf -U lfuse:w:0xFF:m -U hfuse:w:0xDE:m -U efuse:w:0xFD:m  

# do not verify after write
upload_fast:
	avrdude  -V -v -F -c usbasp -p m328p -Uflash:w:target/avr-atmega328p/release/bare_metal.elf -U lfuse:w:0xFF:m -U hfuse:w:0xDE:m -U efuse:w:0xFD:m  

# produces assembly from .elf file
assembly_code:
	avr-objdump -S ./target/avr-atmega328p/release/bare_metal.elf > ./target/avr-atmega328p/release/bare_metal.asm

size:
	@$(AVR_SIZE) --format=sysv $(BUILD_DIR)/bare_metal.elf
	@$(AVR_SIZE) --mcu=$(MCU) $(BUILD_DIR)/bare_metal.elf
	@$(AVR_SIZE) -C --mcu=$(MCU) $(BUILD_DIR)/bare_metal.elf
	

# See in google: "Gource" it is a tool to graphically visualize a repository
gource:
	gource  --auto-skip-seconds 1 --seconds-per-day 1 --default-user-image .\temp\fvilante.JPG --highlight-users --highlight-dirs --title Projeto_Julia_Rust_AVR --key --fullscreen

# NOT TESTED YET. TODO: TEST IT! CAUTION TO NOT PERMANENT DELETE SOME USEFUL DATA
clean:
	rmdir -Force ./target

# Not implemented yet, but something like following should work:
# avr-objcopy -O ihex input.elf output.hex
make_hex_file:

# To install tokei type: `cargo install --git https://github.com/XAMPPRocky/tokei.git tokei`
statistics:
	tokei --exclude "assets/"
	