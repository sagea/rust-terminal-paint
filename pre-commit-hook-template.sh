#!/bin/sh

set -eu

echo "Formatting Validation ⏳"
if ! cargo fmt -- --check
then
    exit 1
fi
echo "Formatting OK ✅"

echo "Clippy Validation"
if ! cargo clippy --all-targets -- -D warnings
then
    echo "There are some clippy issues."
    exit 1
fi
echo "Clippy OK ✅"