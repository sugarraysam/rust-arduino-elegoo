#!/bin/bash

# clean all builds
for dir in *; do
    if [ -d "${dir}" ]; then
        cd "${dir}"
        cargo clean >/dev/null 2>&1 || true
        cd ../
    fi
done
