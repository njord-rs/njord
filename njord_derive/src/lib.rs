extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

/// Derives the `Table` trait for a struct.
///
/// This procedural macro generates implementations of the `Table` trait for a struct.
/// The `Table` trait provides methods for working with database tables.
///
/// # Example
///
/// ```rust
/// use njord_derive::Table;
/// use njord::table::Table;
/// #[derive(Table)]
/// struct MyTable {
///     id: i32,
///     name: String,
/// }
/// ```
///
/// This macro will generate implementations for `get_name`, `get_columns`, and `get_column_fields`
/// based on the struct's field names and types.
#[proc_macro_derive(Table)]
pub fn table_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut name_stream = TokenStream2::default();
    let mut columns_stream = TokenStream2::default();
    let mut column_fields_stream = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let field_names = named.iter().map(|f| &f.ident);
            let field_names_clone = field_names.clone();
            let field_strings = field_names.clone().map(|f| f.as_ref().unwrap().to_string());

            // implement the get_name() function
            name_stream.extend::<TokenStream2>(quote! {
                fn get_name(&self) -> &str {
                    stringify!(#ident)
                }
            });

            // implement the get_columns() function
            columns_stream.extend::<TokenStream2>(quote! {
                fn get_columns(&self) -> std::collections::HashMap<String, String> {
                    let mut columns = std::collections::HashMap::new();
                    #(
                        columns.insert(
                            #field_strings.to_string(),
                            format!("{:?}", self.#field_names_clone)
                        );
                    )*
                    columns
                }
            });

            // implement the get_column_fields() function
            column_fields_stream.extend::<TokenStream2>(quote! {
                fn get_column_fields(&self) -> Vec<String> {
                    vec![#(stringify!(#field_names).to_string()),*]
                }
            });
        }
    };

    let output = quote! {
        impl Table for #ident {
            #name_stream
            #columns_stream
            #column_fields_stream
        }
    };

    output.into()
}
