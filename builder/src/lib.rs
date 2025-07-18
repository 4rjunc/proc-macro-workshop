use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the struct name
    let struct_name = &input.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", struct_name), struct_name.span());

    // getting the fields names
    // "I only know how to use pieces with names on them"
    // A list of all the fields in your struct, separated by commas.
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only struct with named fields are supported"),
        },
        _ => panic!("Only struct are supported"),
    };

    // Making storage boxes
    // Each box has a label and can be empty or have the piece inside
    let builder_fields = fields.iter().map(|field| {
        let name = &field.ident; // The Field Name
        let ty = &field.ty; // The Field Type
        quote! { #name: Option<#ty>}
    });

    // helper functions
    // You create special tools (methods) to put pieces in boxes
    let setters = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub fn #name(&mut self, value: #ty) -> &mut Self {
                self.#name = Some(value);
                self
            }
        }
    });
    // making labels
    let field_names = fields.iter().map(|field| &field.ident);

    // final builder
    let build_fields = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: self.#name.take().expect("Field not set")
        }
    });

    // creating the instruction manual
    let expanded = quote! {
        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }

        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            pub fn new() -> Self {
                Self {
                    #(#field_names: None,)* // repeats the contents for each item in the iterator.
                }
            }

            #(#setters)*

            pub fn build(&mut self) -> #struct_name {
                #struct_name {
                    #(#build_fields,)*
                }
            }
        }
    };
    TokenStream::from(expanded)
}
