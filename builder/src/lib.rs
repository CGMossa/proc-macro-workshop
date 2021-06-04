use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DataStruct, DeriveInput, Field, Ident, Path, PathSegment, TypePath};

#[allow(unused_imports)]
use itertools::Itertools;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // let builder_struct_error_ident = format_ident!("{}BuilderError", ident);
    let DeriveInput {
        attrs: _,
        data,
        generics: _,
        ident,
        vis,
    } = input;
    let builder_struct_ident = format_ident!("{}Builder", ident);

    let fields = match data {
        syn::Data::Struct(DataStruct { fields, .. }) => fields,
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };
    let optional_fields_iter = fields.clone().into_iter().filter_map(
        |Field {
             attrs: _,
             vis,
             ident,
             colon_token: _,
             mut ty,
         }| {
            if match ty {
                syn::Type::Path(TypePath {
                    path: Path { ref segments, .. },
                    ..
                }) => match segments.first() {
                    Some(PathSegment { ident, arguments }) => {
                        if ident == &Ident::new("Option", Span::call_site()) {
                            match arguments {
                                syn::PathArguments::None => todo!(),
                                syn::PathArguments::Parenthesized(_) => todo!(),
                                syn::PathArguments::AngleBracketed(angle_bracketed) => {
                                    match angle_bracketed.args.first() {
                                        Some(first) => match first {
                                            syn::GenericArgument::Lifetime(_) => todo!(),
                                            syn::GenericArgument::Type(inner_type) => {
                                                ty = inner_type.clone();
                                                true
                                            }
                                            syn::GenericArgument::Binding(_) => todo!(),
                                            syn::GenericArgument::Constraint(_) => todo!(),
                                            syn::GenericArgument::Const(_) => todo!(),
                                        },
                                        None => true,
                                    }
                                }
                            }
                        } else {
                            false
                        }
                    }
                    None => {
                        todo!()
                    }
                },
                _ => false,
                // syn::Type::Array(_) => todo!(),
                // syn::Type::BareFn(_) => todo!(),
                // syn::Type::Group(_) => todo!(),
                // syn::Type::ImplTrait(_) => todo!(),
                // syn::Type::Infer(_) => todo!(),
                // syn::Type::Macro(_) => todo!(),
                // syn::Type::Never(_) => todo!(),
                // syn::Type::Paren(_) => todo!(),
                // syn::Type::Ptr(_) => todo!(),
                // syn::Type::Reference(_) => todo!(),
                // syn::Type::Slice(_) => todo!(),
                // syn::Type::TraitObject(_) => todo!(),
                // syn::Type::Tuple(_) => todo!(),
                // syn::Type::Verbatim(_) => todo!(),
            } {
                // eprintln!("{:?} <-> {:#?}", ident, ty);
                Some((vis, ident.expect("doesn't support unnamed fields"), ty))
            } else {
                None
            }
        },
    );
    let optional_field_vis = optional_fields_iter.clone().map(|x| x.0).collect_vec();
    let optional_field_ident = optional_fields_iter.clone().map(|f| f.1).collect_vec();
    let optional_field_types = optional_fields_iter.map(|x| x.2).collect_vec();

    let fields_iter = fields.into_iter().filter_map(
        |Field {
             attrs: _,
             vis,
             ident,
             colon_token: _,
             ty,
         }| {
            if match ty {
                syn::Type::Path(TypePath {
                    path: Path { ref segments, .. },
                    ..
                }) => match segments.first() {
                    Some(PathSegment { ident, .. }) => {
                        ident == &Ident::new("Option", Span::call_site())
                    }
                    None => {
                        todo!()
                    }
                },
                _ => false,
                // syn::Type::Array(_) => todo!(),
                // syn::Type::BareFn(_) => todo!(),
                // syn::Type::Group(_) => todo!(),
                // syn::Type::ImplTrait(_) => todo!(),
                // syn::Type::Infer(_) => todo!(),
                // syn::Type::Macro(_) => todo!(),
                // syn::Type::Never(_) => todo!(),
                // syn::Type::Paren(_) => todo!(),
                // syn::Type::Ptr(_) => todo!(),
                // syn::Type::Reference(_) => todo!(),
                // syn::Type::Slice(_) => todo!(),
                // syn::Type::TraitObject(_) => todo!(),
                // syn::Type::Tuple(_) => todo!(),
                // syn::Type::Verbatim(_) => todo!(),
            } {
                None
            } else {
                Some((vis, ident.expect("doesn't support unnamed fields"), ty))
            }
        },
    );
    let field_vis = fields_iter.clone().map(|x| x.0).collect_vec();
    let field_ident = fields_iter.clone().map(|f| f.1).collect_vec();
    let field_types = fields_iter.map(|x| x.2).collect_vec();

    let tokens = quote! {
        use anyhow::{Result, anyhow};

        #vis struct #builder_struct_ident {
            #(
                #field_vis #field_ident: Option<#field_types>
            ),*,
            #(
                #optional_field_vis #optional_field_ident: Option<#optional_field_types>
            ),*
        }

        impl #builder_struct_ident {

            #(
                /// Set the `#field_ident` field.
                #field_vis fn #field_ident(&mut self, #field_ident: #field_types) -> &mut Self {
                    self.#field_ident = Some(#field_ident);
                    self
                }
            )*

            #(
                /// Set the optional field `#optional_field_ident`.
                #optional_field_vis fn #optional_field_ident(&mut self, #optional_field_ident: #optional_field_types) -> &mut Self {
                    self.#optional_field_ident = Some(#optional_field_ident);
                    self
                }
            )*

        }

        impl #builder_struct_ident {
            pub fn build(&mut self) -> Result<#ident> {
                Ok(
                    #ident {
                        #(
                            #field_ident: self.#field_ident
                                .clone()
                                .ok_or(anyhow!("field `#field_ident` is missing"))?
                        ),*,
                        #(
                            #optional_field_ident: self.#optional_field_ident.clone()
                        ),*
                    }
                )
            }
        }

        impl #ident {
            #vis fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    #( #field_ident: None ),*,
                    #( #optional_field_ident: None ),*
                }
            }
        }

    };

    TokenStream::from(tokens)
}
