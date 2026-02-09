mod model;
use proc_macro::TokenStream;
use model::parse_model;
use syn::{DeriveInput, parse_macro_input};
#[proc_macro_derive(Model,attributes(table,Column))]
pub fn model_derive(input:TokenStream)->TokenStream{
    let mut ast = parse_macro_input!(input as DeriveInput);
    let model =  match parse_model(&mut ast){
        Ok(table) => table,
        Err(e) =>   return e.into_compile_error().into(),
    };
    
    "".parse().unwrap()
}