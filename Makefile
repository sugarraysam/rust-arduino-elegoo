# Default ATmega328 architecture
export ARCH := avr-unknown-gnu-atmega328
export AVR_PART := atmega328p
export AVR_PROGRAMMER := arduino
export SERIAL_PORT := /dev/ttyACM0
# flash OR eeprom
export MEMTYPE := flash


.PHONY: build
build:
	@./scripts/make.sh build

.PHONY: flash
flash:
	@./scripts/make.sh flash

.PHONY: clean
clean:
	@./scripts/clean.sh
