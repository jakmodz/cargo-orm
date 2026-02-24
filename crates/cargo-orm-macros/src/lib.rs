mod generator;
mod model;
mod utils;
use crate::generator::generate_impl;
use crate::model::TableData;
use model::parse_model;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Model, attributes(table, Column, PrimaryKey))]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let model = match parse_model(&mut ast) {
        Ok(table) => table,
        Err(e) => return e.into_compile_error().into(),
    };

    generate_impl(&model)
}
