// Copyright © 2021
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use pipewriter::*;

pipewriter_macro!("pipetest/shader/simple/src/simple.rs");

#[test]
fn load_simple_shader() {
    const SHADERS: &[u8] = include_bytes!(env!("simple_shader.spv"));
    assert!(!SHADERS.is_empty());
}

#[test]
fn build_simple_shader() {
    let _main_pipeline = PipelineMain {};
    let _secondary_pipeline = PipelineSecondary {};
    assert!(1 == 1);
}
