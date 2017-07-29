extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

/// Derive a `from_int` method for the given enum.
#[proc_macro_derive(FromInt)]
pub fn derive_format_args(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let ast = syn::parse_derive_input(&string).unwrap();
    implement(&ast).parse().unwrap()
}

fn implement(ast: &syn::DeriveInput) -> quote::Tokens {
    let ident = &ast.ident;
    let (ident2, ident3, ident4) = (ident, ident, ident);
    let variants = match ast.body {
        syn::Body::Enum(ref variants) => variants,
        _ => panic!("#[derive(FromInt)] only available for enums")
    };

    let names = variants.iter().map(|v| &v.ident).collect::<Vec<_>>();
    let names2 = names.clone();

    quote! {
        impl #ident {
            pub fn from_int(value: usize) -> Option<#ident2> {
                use self::#ident3::*;
                #(
                    if value == (#names as usize) { Some(#names2) } else
                )* { ::std::option::Option::None }
            }
        }
    }
}
