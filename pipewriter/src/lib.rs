// Copyright © 2021
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

extern crate proc_macro;

use proc_macro::*;

use quote::quote;

mod util;
use util::*;

mod shader;
use shader::*;

mod module;
use module::*;

#[proc_macro]
pub fn pipewriter_macro(input: TokenStream) -> TokenStream {
    let shader_crate = input.to_string().replace("\"", "");
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let crate_dir = current_dir.join(&shader_crate);

    let crate_module = CrateModule::new(crate_dir);

    // Build the Pipeline implementation
    gen_pipelines(&crate_module)
}

fn gen_pipelines(crate_module: &CrateModule) -> TokenStream {
    let crate_name: proc_macro2::TokenStream = format!("Crate{}", crate_module.name.to_camelcase())
        .parse()
        .unwrap();
    let shader_spv = format!("{}.spv", crate_module.name.replace('-', "_"));

    let pipelines = get_pipelines(&crate_module.file);

    let pipeline_names = pipelines.iter().map(|m| {
        let pipeline_name = format!("Pipeline{}", m.name.to_camelcase());
        pipeline_name
            .parse::<proc_macro2::TokenStream>()
            .expect("Failed to parse shader name")
    });

    let mut gen = quote! {
        use std::rc::Rc;
        use ash::Device;
        use vkr_core::ShaderModule;
    };

    for pipeline in pipeline_names {
        let pipeline_gen = quote! {
            pub struct #pipeline {
            }

            impl #pipeline {
                pub fn new(shader_module: &ShaderModule) -> Self {
                    Self {
                    }
                }
            }
        };

        gen.extend(pipeline_gen);
    }

    let pipeline_vars = pipelines.iter().map(|m| {
        m.name
            .to_lowercase()
            .parse::<proc_macro2::TokenStream>()
            .expect("Failed to parse shader name")
    });

    let pipeline_defs = pipelines.iter().map(|m| {
        let pipeline_name = format!(
            "{}: Pipeline{}",
            m.name.to_lowercase(),
            m.name.to_camelcase()
        );
        pipeline_name
            .parse::<proc_macro2::TokenStream>()
            .expect("Failed to parse shader name")
    });

    let pipeline_vars_impl = pipelines.iter().map(|m| {
        let pipeline_name = format!(
            "let {} = Pipeline{}::new(&shader_module)",
            m.name.to_lowercase(),
            m.name.to_camelcase()
        );
        pipeline_name
            .parse::<proc_macro2::TokenStream>()
            .expect("Failed to parse shader name")
    });

    let crate_gen = quote! {
        pub struct #crate_name {
            shader_module: ShaderModule,
            pub #( #pipeline_defs, )*
        }

        impl #crate_name {
            pub fn new(device: &Rc<Device>) -> Self {
                const CODE: &[u8] = include_bytes!(env!(#shader_spv));
                let shader_module = ShaderModule::new(device, CODE);
                #( #pipeline_vars_impl; )*
                Self {
                    shader_module,
                #( #pipeline_vars, )*
                }
            }
        }
    };
    gen.extend(crate_gen);

    gen.into()
}

/// Collects all the pipelines found in a shader file
fn get_pipelines(file: &syn::File) -> Vec<Pipeline> {
    let mut pipelines = vec![];

    let functions = file
        .items
        .iter()
        .filter_map(|i| inner_value!(i, syn::Item::Fn(f) => f));

    // Go through all the functions of the file
    for func in functions {
        if let Some(spirv) = get_spirv(func) {
            let shader_type = get_shader_type(&spirv);
            if let Some(ShaderType::Fragment) = shader_type {
                // Extract prefix of function
                let prefix = get_prefix(&func.sig.ident.to_string());
                // Convert to camelcase and use it to name the pipeline
                let name = prefix.to_camelcase();
                pipelines.push(Pipeline::new(name));
            }
        }
    }

    pipelines
}

/// Analyzes the attributes of a function, looking for a spirv `MetaList`
fn get_spirv(func: &syn::ItemFn) -> Option<syn::MetaList> {
    func.attrs
        .iter()
        // which are metas
        .filter_map(|attr| attr.parse_meta().ok())
        // which are lists
        .filter_map(|meta| inner_value!(meta, syn::Meta::List(l) => l))
        // which idents are spirv
        .filter(|list| list.path.get_ident().unwrap() == "spirv")
        .next() // and take first
}

/// Analyzes a spirv `MetaList`, looking for vertex and fragment `Path`s
/// and returns the corresponding shader type
fn get_shader_type(spirv: &syn::MetaList) -> Option<ShaderType> {
    for nested in &spirv.nested {
        if let syn::NestedMeta::Meta(meta) = nested {
            if let syn::Meta::Path(path) = meta {
                if let Some(ident) = path.get_ident() {
                    if ident == "vertex" {
                        return Some(ShaderType::Vertex);
                    } else if ident == "fragment" {
                        return Some(ShaderType::Fragment);
                    }
                }
            }
        }
    }
    None
}
