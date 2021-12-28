// Copyright © 2021
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use pipewriter::*;

pipewriter_macro!("tests/shader/simple/src/simple.rs");

#[test]
fn build_simple_shader() {
    let _pipeline = PipelineMain {};
    assert!(1 == 1);
}
