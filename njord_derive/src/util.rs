use syn::Meta;

/// Check if the Default trait is implemented for the struct.
///
/// This function takes a `syn::DeriveInput` as input and checks if the Default trait is
/// implemented for the struct. It analyzes the type parameters and their bounds to determine
/// if the Default trait is present.
///
/// # Parameters
///
/// - `input`: A reference to the `syn::DeriveInput` representing the input struct.
///
/// # Returns
///
/// Returns `true` if the Default trait is implemented for the struct, and `false` otherwise.
pub fn has_default_impl(input: &syn::DeriveInput) -> bool {
    if let syn::Data::Struct(_s) = &input.data {
        let generics = &input.generics;
        return generics.params.iter().any(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                type_param.bounds.iter().any(|bound| {
                    if let syn::TypeParamBound::Trait(tb) = bound {
                        tb.path.is_ident("Default")
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        });
    }
    false
}

/// Extracts the table name from attributes.
///
/// This function searches for the `table_name` attribute in the given attributes and returns
/// the specified table name if found. If the attribute is not present, it returns a default table name.
///
/// # Arguments
///
/// * `attrs` - A slice of `syn::Attribute` representing the attributes of a struct.
///
/// # Returns
///
/// A `String` representing the extracted table name or the default table name if not specified.
pub fn extract_table_name(attrs: &[syn::Attribute]) -> String {
    for attr in attrs {
        if let Some(attr_meta_name) = attr.path().get_ident() {
            if attr_meta_name == "table_name" {
                let attr_meta = &attr.meta;

                match attr_meta {
                    Meta::NameValue(val) => val,
                    _ => panic!("Incorrect format for using the `table_name` attribute."),
                };
            }
        }
    }
    // Default table name if not specified
    "default_table_name".to_string()
}
