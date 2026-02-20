mod model;
mod generator;
mod utils;
use proc_macro::TokenStream;
use model::parse_model;
use syn::{DeriveInput, parse_macro_input};
use crate::model::TableData;
use crate::generator::generate_impl;

#[proc_macro_derive(Model,attributes(table,Column))]
pub fn model_derive(input:TokenStream)->TokenStream{
    let mut ast = parse_macro_input!(input as DeriveInput);
    let model =  match parse_model(&mut ast){
        Ok(table) => table,
        Err(e) =>   return e.into_compile_error().into(),
    };
    
    generate_impl(&model)
}