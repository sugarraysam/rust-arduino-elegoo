#!/bin/bash

function check_serial() {
    if [ ! -c "${SERIAL}" ]; then
        echo "Serial file does not exist '${SERIAL_PORT}'..."
        exit 1
    fi
}

function check_elf() {
    if [ ! -f "${ELF}" ]; then
        echo "No ELF found at '${ELF}'..."
        echo "Please run 'make flash EXAMPLE=<example>'..."
        exit 1
    fi
}

function flash() {
    avrdude -p "${ARCH}" -c "${PROGRAMMER}" -P "${SERIAL}" -U "${MEMTYPE}:w:${ELF}:e"
}

###
### Execution
###
export ELF="target/${ARCH}/release/examples/${EXAMPLE}.elf"
check_serial
check_elf
flash
