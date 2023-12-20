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
