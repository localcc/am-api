use crate::context_container::context_derive;
use crate::resource_property::relation_derive;
use proc_macro::TokenStream;

mod context_container;
mod resource_property;

#[proc_macro_derive(ResourceProperty, attributes(resource_property))]
pub fn resource_property(input: TokenStream) -> TokenStream {
    relation_derive(input)
}

#[proc_macro_derive(Context, attributes(context))]
pub fn context(input: TokenStream) -> TokenStream {
    context_derive(input)
}
