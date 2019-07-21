use std::env;
use std::panic;

use quote::quote;
use syn;
use syn::export::TokenStream;

#[proc_macro_attribute]
pub fn mammoth_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let constructor: syn::Ident = syn::parse(attr).unwrap();
    let ast: syn::ItemStruct = syn::parse(item).unwrap();
    let name = &ast.ident;

    if env::var("MAMMOTH_MODULE").is_ok() {
        panic!("Only one MammothInterface per library is allowed.");
    } else {
        env::set_var("MAMMOTH_MODULE", "impl");
    }

    let result = quote!{
        trait __mammoth_interface: mammoth_setup::MammothInterface {}

        #[no_mangle]
        pub fn __version() -> semver::Version {
            semver::Version::new(0,0,1)
        }

        #[no_mangle]
        pub fn __construct(cfg: Option<toml::Value>) -> #name {
            #constructor(cfg)
        }

        #ast

        impl __mammoth_interface for #name {}
    };

    result.into()
}