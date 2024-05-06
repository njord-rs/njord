//! BSD 3-Clause License
//!
//! Copyright (c) 2024, Marcus Cvjeticanin
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use quote::quote;
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
                    Meta::NameValue(val) => {
                        let expr = &val.value;
                        let expr_token_stream = quote! { #expr };
                        let table_value = expr_token_stream.to_string();

                        return table_value;
                    }
                    _ => panic!("Incorrect format for using the `table_name` attribute."),
                };
            }
        }
    }
    // Default table name if not specified
    "default_table_name".to_string()
}

/// Checks if the given type is an Option<T> type.
///
/// # Arguments
///
/// * `ty` - A reference to the `syn::Type` to be checked.
///
/// # Returns
///
/// A boolean indicating whether the type is an Option<T> type.
/// ```
#[allow(dead_code)]
pub fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        let segments = &type_path.path.segments;
        if segments.len() == 1 && segments.first().unwrap().ident.to_string() == "Option" {
            if let syn::PathArguments::AngleBracketed(args) = &segments.first().unwrap().arguments {
                if args.args.len() == 1 {
                    return true;
                }
            }
        }
    }
    false
}
