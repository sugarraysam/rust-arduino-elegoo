export PROJECT := rust-arduino-elegoo
export ARCH := atmega328p
export PROGRAMMER := arduino
export SERIAL := /dev/ttyACM0
# flash OR eeprom
export MEMTYPE := flash


.PHONY: build
build:
	@cargo build --example $(EXAMPLE) --release

.PHONY: flash
flash:
	@./scripts/flash.sh $(EXAMPLE)

.PHONY: clean
clean:
	@cargo clean

.PHONY: doc
doc:
	@cargo doc --open
