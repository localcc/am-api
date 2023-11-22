//! Resource property attribute

use crate::resource_property::attribute::{
    ContainerMode, ContainerResourcePropertyAttribute, FieldMode, FieldResourcePropertyAttribute,
};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Path};

mod attribute;

const RESOURCE_PROPERTY: &str = "resource_property";

const EXTENSION_TRAIT: &str = "crate::request::extension::ExtensionTrait";
const RELATIONSHIP_TRAIT: &str = "crate::request::relationship::RelationshipTrait";
const VIEW_TRAIT: &str = "crate::request::view::ViewTrait";

pub fn relation_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        attrs: attributes,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let Data::Struct(data_struct) = data else {
        panic!("Only structs are supported");
    };

    let Fields::Named(named) = data_struct.fields else {
        panic!("Only named structs are supported");
    };

    let attribute = attributes
        .iter()
        .find(|e| e.meta.path().is_ident(RESOURCE_PROPERTY))
        .expect("expected resource_property attribute on container")
        .clone();
    let attribute = ContainerResourcePropertyAttribute::new(&attribute);

    let enum_name = attribute.name;
    let enum_body = body_for_enum(&named, attribute.field_mode);
    let enum_def = quote! {
        /// Reflected types
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum #enum_name {
            #enum_body
        }
    };

    let relation_object = attribute.relation_object;
    let trait_name = match attribute.container_mode {
        ContainerMode::Extension => EXTENSION_TRAIT,
        ContainerMode::Relationship => RELATIONSHIP_TRAIT,
        ContainerMode::View => VIEW_TRAIT,
    };
    let trait_name: Path = syn::parse_str(trait_name).unwrap();
    let trait_impl = quote! {
        impl #trait_name for #enum_name {
            fn get_object(&self) -> &'static str {
                #relation_object
            }
        }
    };

    let match_body = body_for_match(&named, attribute.field_mode);
    let display_impl = quote! {
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = match self {
                    #match_body
                };
                write!(f, "{}", string)
            }
        }
    };

    TokenStream::from(quote! {
        #enum_def

        #trait_impl
        #display_impl
    })
}

fn body_for_match(fields: &FieldsNamed, mode: FieldMode) -> proc_macro2::TokenStream {
    for_each_field(fields, mode, |field| {
        let original_name = field
            .ident
            .as_ref()
            .expect("Named fields always have an ident")
            .to_string();

        let name = Ident::new(&original_name.to_case(Case::UpperCamel), Span::call_site());
        let string_name = get_field_attr(&field.attrs)
            .and_then(|e| e.name)
            .map(|e| e.value())
            .unwrap_or_else(|| original_name.to_case(Case::Train).to_lowercase());

        quote! {
            Self::#name => #string_name,
        }
    })
}

fn body_for_enum(fields: &FieldsNamed, mode: FieldMode) -> proc_macro2::TokenStream {
    for_each_field(fields, mode, |field| {
        let comments = extract_doc_comments(&field.attrs);
        let name = Ident::new(
            &field
                .ident
                .clone()
                .expect("named fields always have an ident")
                .to_string()
                .to_case(Case::UpperCamel),
            Span::call_site(),
        );

        quote! {
            #comments
            #name,
        }
    })
}

fn for_each_field<F: Fn(&Field) -> proc_macro2::TokenStream>(
    fields_named: &FieldsNamed,
    mode: FieldMode,
    executor: F,
) -> proc_macro2::TokenStream {
    let streams = fields_named
        .named
        .iter()
        .filter(|e| {
            let attr = get_field_attr(&e.attrs);
            match mode {
                FieldMode::Blacklist => attr.map(|e| !e.skip).unwrap_or(true),
                FieldMode::Whitelist => attr.map(|e| !e.skip && e.whitelisted).unwrap_or(false),
            }
        })
        .map(executor);

    quote! {
        #(#streams)*
    }
}

fn get_field_attr(attrs: &[Attribute]) -> Option<FieldResourcePropertyAttribute> {
    attrs
        .iter()
        .find(|e| e.meta.path().is_ident(RESOURCE_PROPERTY))
        .map(FieldResourcePropertyAttribute::new)
}

fn extract_doc_comments(attributes: &[Attribute]) -> proc_macro2::TokenStream {
    let streams = attributes
        .iter()
        .filter(|e| e.path().is_ident("doc"))
        .map(|e| e.clone().into_token_stream());

    quote! {
        #(#streams)*
    }
}
