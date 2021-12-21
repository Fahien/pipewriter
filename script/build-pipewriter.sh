#!/bin/sh
# Copyright © 2021
# Author: Antonio Caggiano <info@antoniocaggiano.eu>
# SPDX-License-Identifier: MIT

BRANCH=main
REPO=https://raw.githubusercontent.com/EmbarkStudios/rust-gpu

curl $REPO/$BRANCH/rust-toolchain --output rust-toolchain
cargo test
