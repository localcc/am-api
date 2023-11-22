use syn::Attribute;

pub struct ContextPropertyAttribute {
    pub skip: bool,
}

impl ContextPropertyAttribute {
    pub fn new(attribute: &Attribute) -> ContextPropertyAttribute {
        let mut skip = false;

        attribute
            .parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    skip = true;
                    Ok(())
                } else {
                    Err(meta.error("unsupported context property"))
                }
            })
            .unwrap();

        ContextPropertyAttribute { skip }
    }
}
