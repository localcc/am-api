//! Context container derive macro
//!
//! This macro is used to fill context variables inside of a container

use crate::context_container::attribute::ContextPropertyAttribute;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed,
    FieldsUnnamed, Path,
};

mod attribute;

/// Attribute name
const CONTEXT: &str = "context";
/// Trait name
const TRAIT_NAME: &str = "crate::request::context::ContextContainer";
/// Context path
const CONTEXT_PATH: &str = "std::sync::Arc<crate::request::context::RequestContext>";

/// Context container derive macro
pub fn context_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        generics,
        ident: name,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let should_generate_bounds = false;

    let body = match &data {
        syn::Data::Struct(e) => {
            generate_body_for_struct(&name, &generics, e, should_generate_bounds, &[TRAIT_NAME])
        }
        syn::Data::Enum(e) => {
            generate_body_for_enum(&name, &generics, e, should_generate_bounds, &[TRAIT_NAME])
        }
        syn::Data::Union(_) => panic!("This macro cannot be used on unit structs!"),
    };

    TokenStream::from(body)
}

fn generate_body_for_struct(
    name: &Ident,
    generics: &syn::Generics,
    data_struct: &DataStruct,
    should_generate_bounds: bool,
    traits: &[&str],
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, old_where_clause) = generics.split_for_impl();

    let new_where_clause = match should_generate_bounds {
        true => {
            let new_where_clause = add_trait_bounds_for_all_struct_fields(data_struct, traits);
            let mut new_where_clause: syn::WhereClause = syn::parse2(new_where_clause).unwrap();
            if let Some(old_where_clause) = old_where_clause {
                new_where_clause
                    .predicates
                    .extend(old_where_clause.predicates.clone());
            }
            quote! { #new_where_clause }
        }
        false => quote! { #old_where_clause },
    };

    let body = match &data_struct.fields {
        Fields::Named(e) => body_for_struct_named_fields(e),
        Fields::Unnamed(e) => body_for_struct_unnamed_fields(e),
        Fields::Unit => quote! {},
    };

    let trait_name: Path = syn::parse_str(TRAIT_NAME).unwrap();
    let context_path: Path = syn::parse_str(CONTEXT_PATH).unwrap();

    quote! {
        impl #impl_generics #trait_name for #name #ty_generics #new_where_clause {
            fn set_context(&mut self, context: #context_path) {
                #body
            }
        }
    }
}

fn generate_body_for_enum(
    name: &Ident,
    generics: &syn::Generics,
    data_enum: &DataEnum,
    should_generate_bounds: bool,
    traits: &[&str],
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, old_where_clause) = generics.split_for_impl();

    let new_where_clause = match should_generate_bounds {
        true => {
            let new_where_clause = add_trait_bounds_for_all_enum_variants(data_enum, traits);
            let mut new_where_clause: syn::WhereClause = syn::parse2(new_where_clause).unwrap();
            if let Some(old_where_clause) = old_where_clause {
                new_where_clause
                    .predicates
                    .extend(old_where_clause.predicates.clone());
            }
            quote! { #new_where_clause }
        }
        false => quote! { #old_where_clause },
    };

    let body = body_for_enum_variants(data_enum);

    let trait_name: Path = syn::parse_str(TRAIT_NAME).unwrap();
    let context_path: Path = syn::parse_str(CONTEXT_PATH).unwrap();

    quote! {
        impl #impl_generics #trait_name for #name #ty_generics #new_where_clause {
            fn set_context(&mut self, context: #context_path) {
                match self {
                    #body
                    _ => {}
                }
            }
        }
    }
}

fn add_trait_bounds_for_all_struct_fields(
    data_struct: &DataStruct,
    traits: &[&str],
) -> proc_macro2::TokenStream {
    let traits: syn::Type = syn::parse_str(&traits.join(" + ")).unwrap();

    let bounds = for_each_field_type(&data_struct.fields, |field_type| {
        quote! {
            #field_type: #traits,
        }
    });

    quote! {
        where #bounds
    }
}

fn add_trait_bounds_for_all_enum_variants(
    data_enum: &DataEnum,
    traits: &[&str],
) -> proc_macro2::TokenStream {
    let traits: syn::Type = syn::parse_str(&traits.join(" + ")).unwrap();

    let bounds =
        proc_macro2::TokenStream::from_iter(data_enum.variants.iter().map(|e| match &e.fields {
            Fields::Unnamed(unnamed_fields) => {
                for_each_unnamed_field_type(unnamed_fields, |field_type| {
                    quote! {
                        #field_type: #traits,
                    }
                })
            }
            Fields::Named(named_fields) => for_each_named_field_type(named_fields, |field_type| {
                quote! {
                    #field_type: #traits,
                }
            }),
            Fields::Unit => quote! {},
        }));

    quote! {
        where #bounds
    }
}

fn body_for_struct_named_fields(fields_named: &FieldsNamed) -> proc_macro2::TokenStream {
    let streams = fields_named
        .named
        .iter()
        .filter(|e| get_field_attr(&e.attrs).map(|e| !e.skip).unwrap_or(true))
        .map(|e| {
            let context_container = e.ident.as_ref().unwrap();
            quote! {
                self.#context_container.set_context(context.clone())
            }
        });
    quote! {
        #(#streams;)*
    }
}

fn body_for_struct_unnamed_fields(fields_unnamed: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let streams = fields_unnamed
        .unnamed
        .iter()
        .filter(|e| get_field_attr(&e.attrs).map(|e| !e.skip).unwrap_or(true))
        .enumerate()
        .map(|(index, _)| {
            let index = syn::Index::from(index);
            quote! {
                self.#index.set_context(context.clone())
            }
        });

    quote! {
        #(#streams;)*
    }
}

fn body_for_enum_variants(data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let variant_streams = data_enum.variants.iter().map(|e| match &e.fields {
        Fields::Named(named) => body_for_named_enum_variant(&e.ident, named),
        Fields::Unnamed(unnamed) => body_for_unnamed_enum_variant(&e.ident, unnamed),
        Fields::Unit => quote! {},
    });

    quote! {
        #(#variant_streams,)*
    }
}

fn body_for_named_enum_variant(
    variant_name: &Ident,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let fields = fields
        .named
        .iter()
        .filter(|e| get_field_attr(&e.attrs).map(|e| !e.skip).unwrap_or(true))
        .map(|e| e.ident.clone().expect("named fields always have an ident"));

    let fields_ = fields.clone();

    quote! {
        Self::#variant_name { #(#fields,)* } => {
            #(#fields_.set_context(context.clone());)*
        }
    }
}

fn body_for_unnamed_enum_variant(
    variant_name: &Ident,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    const VARIABLE_NAMES: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let fields = fields
        .unnamed
        .iter()
        .filter(|e| get_field_attr(&e.attrs).map(|e| !e.skip).unwrap_or(true))
        .enumerate()
        .map(|(index, _)| format_ident!("{}", VARIABLE_NAMES[index]));

    let fields_ = fields.clone();

    quote! {
        Self::#variant_name #((#fields,))* => {
            #(#fields_.set_context(context.clone());)*
        }
    }
}

fn for_each_field_type<F: Fn(&syn::Type) -> proc_macro2::TokenStream>(
    fields: &Fields,
    executor: F,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => for_each_named_field_type(fields_named, executor),
        Fields::Unnamed(fields_unnamed) => for_each_unnamed_field_type(fields_unnamed, executor),
        Fields::Unit => quote! {},
    }
}

fn for_each_named_field_type<F: Fn(&syn::Type) -> proc_macro2::TokenStream>(
    fields_named: &FieldsNamed,
    executor: F,
) -> proc_macro2::TokenStream {
    let streams = fields_named
        .named
        .iter()
        .filter(|e| get_field_attr(&e.attrs).map(|e| !e.skip).unwrap_or(true))
        .map(|e| executor(&e.ty));

    quote! {
        #(#streams)*
    }
}

fn for_each_unnamed_field_type<F: Fn(&syn::Type) -> proc_macro2::TokenStream>(
    fields_unnamed: &FieldsUnnamed,
    executor: F,
) -> proc_macro2::TokenStream {
    let streams = fields_unnamed
        .unnamed
        .iter()
        .filter(|e| get_field_attr(&e.attrs).map(|e| !e.skip).unwrap_or(true))
        .map(|e| executor(&e.ty));

    quote! {
        #(#streams)*
    }
}

fn get_field_attr(attrs: &[Attribute]) -> Option<ContextPropertyAttribute> {
    attrs
        .iter()
        .find(|e| e.meta.path().is_ident(CONTEXT))
        .map(ContextPropertyAttribute::new)
}
