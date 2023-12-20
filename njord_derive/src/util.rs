/// Check if the Default trait is implemented for the struct.
pub fn has_default_impl(input: &syn::DeriveInput) -> bool {
    if let syn::Data::Struct(s) = &input.data {
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
