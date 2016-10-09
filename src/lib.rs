#![feature(rustc_macro, rustc_macro_lib)]
#![cfg(not(test))]

extern crate rustc_macro;
extern crate syn;
extern crate synstructure;
#[macro_use]
extern crate quote;

use rustc_macro::TokenStream;
use synstructure::{each_field, BindStyle};

#[rustc_macro_derive(Abomonation)]
pub fn derive_abomonation(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let mut ast = syn::parse_macro_input(&source).unwrap();

    // Generate the Entomb, Embalm, and Exhume match bodies
    let entomb = each_field(&mut ast, BindStyle::Ref, |bi| {
        quote! {
        ::abomonation::Abomonation::entomb(#bi, _writer);
    }
    });
    let embalm = each_field(&mut ast, BindStyle::RefMut, |bi| {
        quote! {
        ::abomonation::Abomonation::embalm(#bi);
    }
    });
    let exhume = each_field(&mut ast, BindStyle::RefMut, |bi| {
        quote! {
        let temp = bytes;
        let exhume_result = ::abomonation::Abomonation::exhume(#bi, temp);
        bytes = if let Some(bytes) = exhume_result {
            bytes
        } else {
            return None
        };
    }
    });

    // Build the output tokens
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let result = quote! {
        // Original struct
        #ast

        impl #impl_generics ::abomonation::Abomonation for #name #ty_generics #where_clause {
            #[inline] unsafe fn entomb(&self, _writer: &mut Vec<u8>) {
                match *self { #entomb }
            }
            #[inline] unsafe fn embalm(&mut self) {
                match *self { #embalm }
            }
            #[inline] unsafe fn exhume<'a,'b>(&'a mut self, mut bytes: &'b mut [u8])
                                              -> Option<&'b mut [u8]> {
                match *self { #exhume }
                Some(bytes)
            }
        }
    };

    // Generate the final value as a TokenStream and return it
    result.to_string().parse().unwrap()
}
