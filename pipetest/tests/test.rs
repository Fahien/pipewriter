// Copyright © 2021
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use pipewriter::*;
use vkr_core::{Ctx, Dev};

pipewriter_macro!("pipetest/shader/simple");

#[test]
fn load_simple_shader() {
    const SHADERS: &[u8] = include_bytes!(env!("simple_shader.spv"));
    assert!(!SHADERS.is_empty());
}

#[test]
fn build_simple_shader() {
    let ctx = Ctx::builder().build();
    let dev = Dev::new(&ctx, None);

    let shader_crate = CrateSimpleShader::new(&dev.device);
    let _main_pipeline = &shader_crate.main;
    let _secondary_pipeline = &shader_crate.secondary;

    assert!(1 == 1);
}
