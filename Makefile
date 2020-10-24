# Default ATmega328 architecture
export ARCH := avr-unknown-gnu-atmega328
export AVR_PART := atmega328p
export AVR_PROGRAMMER := arduino
export SERIAL_PORT := /dev/ttyACM0
# flash OR eeprom
export MEMTYPE := flash


.PHONY: build
build:
	@cargo build -Z build-std=core --target $(ARCH) --release

# TODO read binary name as arg
.PHONY: flash
flash:
	@avrdude -p $(AVR_PART) -c $(AVR_PROGRAMMER) -P $(SERIAL_PORT) \
		-U $(MEMTYPE):w:target/$(ARCH)/release/$(BINARY).elf:e
