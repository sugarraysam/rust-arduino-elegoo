#!/bin/bash

function check_dir() {
    if [ -z "${DIR}" ] || [ ! -d "${DIR}" ]; then
        echo "Please provide a valid directory, got '${DIR}'..."
        exit 1
    fi
}

function check_serial() {
    if [ ! -c "${SERIAL_PORT}" ]; then
        echo "Serial file does not exist '${SERIAL_PORT}'..."
        exit 1
    fi
}

function set_binary() {
    BASEPATH="target/${ARCH}/release"
    NAME=$(ls "${BASEPATH}" | grep elf)
    export BINARY="${BASEPATH}/${NAME}"
    if [ ! -f "${BINARY}" ]; then
        echo "No binary found at '${BINARY}'..."
        exit 1
    fi
}

###
### Execution
###
check_dir
cd "${DIR}" || exit 1
rustup override set nightly >/dev/null 2>&1

action="${1}"
echo "Executing action: '${action}'..."

case "${action}" in
"build")
    cargo build -Z build-std=core --target "${ARCH}" --release
    ;;
"flash")
    set_binary
    check_serial
    avrdude -p "${AVR_PART}" -c "${AVR_PROGRAMMER}" -P "${SERIAL_PORT}" \
        -U "${MEMTYPE}:w:${BINARY}:e"
    ;;
*)
    echo "usage: $0 [ build | flash ]"
    exit 1
    ;;
esac
