use proc_macro2::Ident;
use syn::{Attribute, LitStr};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ContainerMode {
    Extension,
    Relationship,
    View,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum FieldMode {
    #[default]
    Blacklist,
    Whitelist,
}

pub struct ContainerResourcePropertyAttribute {
    pub name: Ident,
    pub relation_object: LitStr,
    pub container_mode: ContainerMode,
    pub field_mode: FieldMode,
}

impl ContainerResourcePropertyAttribute {
    pub fn new(attribute: &Attribute) -> ContainerResourcePropertyAttribute {
        let mut name = None;
        let mut relation_name = None;
        let mut container_mode = None;
        let mut field_mode = FieldMode::default();

        attribute
            .parse_nested_meta(|meta| {
                if meta.path.is_ident("object") {
                    relation_name = Some(meta.value()?.parse()?);
                    Ok(())
                } else if meta.path.is_ident("whitelist") {
                    field_mode = FieldMode::Whitelist;
                    Ok(())
                } else if meta.path.is_ident("blacklist") {
                    field_mode = FieldMode::Blacklist;
                    Ok(())
                } else if meta.path.is_ident("extension") {
                    container_mode = Some(ContainerMode::Extension);
                    Ok(())
                } else if meta.path.is_ident("relationship") {
                    container_mode = Some(ContainerMode::Relationship);
                    Ok(())
                } else if meta.path.is_ident("view") {
                    container_mode = Some(ContainerMode::View);
                    Ok(())
                } else {
                    name = Some(
                        meta.path
                            .get_ident()
                            .cloned()
                            .ok_or_else(|| meta.error("missing struct name"))?,
                    );
                    Ok(())
                }
            })
            .unwrap();

        ContainerResourcePropertyAttribute {
            name: name.expect("missing struct name"),
            relation_object: relation_name.expect("missing resource_property object"),
            container_mode: container_mode.expect("missing container mode"),
            field_mode,
        }
    }
}

pub struct FieldResourcePropertyAttribute {
    pub name: Option<LitStr>,
    pub skip: bool,
    pub whitelisted: bool,
}

impl FieldResourcePropertyAttribute {
    pub fn new(attribute: &Attribute) -> FieldResourcePropertyAttribute {
        let mut name = None;
        let mut skip = false;
        let mut whitelisted = false;

        attribute
            .parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    skip = true;
                    Ok(())
                } else if meta.path.is_ident("whitelist") {
                    whitelisted = true;
                    Ok(())
                } else if meta.path.is_ident("name") {
                    name = Some(meta.value()?.parse()?);
                    Ok(())
                } else {
                    Err(meta.error("unsupported reflect property"))
                }
            })
            .unwrap();

        FieldResourcePropertyAttribute {
            name,
            skip,
            whitelisted,
        }
    }
}
