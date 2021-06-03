use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{DeriveInput, Type, parse_macro_input};

use itertools::Itertools;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // let builder_struct_error_ident = format_ident!("{}BuilderError", ident);
    let DeriveInput {
        attrs,
        data,
        generics,
        ident,
        vis,
    } = input;
    let builder_struct_ident = format_ident!("{}Builder", ident);

    let fields = match data {
        syn::Data::Struct(ds) => {
            ds.fields
            //     match ds.fields {
            //     syn::Fields::Named(fields) => fields,
            //     syn::Fields::Unnamed(_) => todo!(),
            //     syn::Fields::Unit => todo!(),
            // }
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    let (field_names, field_types): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .map(|f| {
            let Field { } = f;
        })
        .unzip();

    let tokens = quote! {

        use anyhow::Result;

        pub struct #builder_struct_ident {
            #(
                #field_names: Option<#field_types>
            ),*
        }

        // impl #builder_struct_ident {

        //     /// Set the `executable` field
        //     pub fn executable(&mut self, executable: String) -> &mut Self {
        //         self.executable = Some(executable);
        //         self
        //     }


        //     /// Set the `args` field
        //     pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        //         self.args = Some(args);
        //         self
        //     }


        //     /// Set the `env` field
        //     pub fn env(&mut self, env: Vec<String>) -> &mut Self {
        //         self.env = Some(env);
        //         self
        //     }


        //     /// Set the `current_dir` field
        //     pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        //         self.current_dir = Some(current_dir);
        //         self
        //     }

        // }

        // impl #builder_struct_ident {
        //     pub fn build(&mut self) -> Result<#ident> {
        //         unimplemented!()
        //     }
        // }

        impl #ident {
            pub fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    #( #field_names: None ),*
                }
            }
        }

    };

    TokenStream::from(tokens)
}
