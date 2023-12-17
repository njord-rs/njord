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
///     name: String,
///     price: f64,
///     in_stock: bool
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
    let mut column_values_stream = TokenStream2::default();
    let mut set_column_values_stream = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let field_names = named.iter().map(|f| &f.ident);
            let field_names_clone = field_names.clone();
            let field_names_clone2 = field_names.clone();
            let field_types = named.iter().map(|f| &f.ty);
            let field_types_clone = named.iter().map(|f| &f.ty);
            let field_values = named.iter().map(|f| {
                let field_name = &f.ident;
                quote! { self.#field_name.to_string() }
            });

            // implement the get_name() function
            name_stream.extend::<TokenStream2>(quote! {
                fn get_name(&self) -> &str {
                    stringify!(#ident)
                }
            });

            // implement the get_columns() function
            columns_stream.extend(quote! {
                fn get_columns(&self) -> std::collections::HashMap<String, String> {
                    let mut columns = std::collections::HashMap::new();
                    #(
                        let column_type = match stringify!(#field_types) {
                            "i64" | "i32" | "i16" | "i8" | "u64" | "u32" | "u16" | "u8" | "usize" => "INTEGER",
                            "String" => "TEXT",
                            "f64" | "f32" => "REAL",
                            "Vec<u8>" => "BLOB",
                            "Option<i64>" | "Option<i32>" | "Option<i16>" | "Option<i8>" | "Option<u64>" | "Option<u32>" | "Option<u16>" | "Option<u8>" | "Option<usize>" => "INTEGER",
                            "Option<String>" => "TEXT",
                            "Option<f64>" | "Option<f32>" => "REAL",
                            "Option<Vec<u8>>" => "BLOB",
                            "bool" => "TEXT",
                            
                            // for nested structs, we include their columns
                            stringify!($field_types_clone) if $field_types_clone: Table => {
                                columns.extend_from_hashmap(<$field_types_clone as Table>::get_columns(&self.$field_names_clone));
                            }

                            _ => {
                                eprintln!("Warning: Unknown data type for column '{}'", stringify!(#field_names));
                                "UNKNOWN_TYPE"
                            }
                        };
                        columns.insert(
                            stringify!(#field_names).to_string(),
                            column_type.to_string(),
                        );
                    )*
                    columns
                }
            });

            // implement the get_column_fields() function
            column_fields_stream.extend(quote! {
                fn get_column_fields(&self) -> Vec<String> {
                    vec![#(stringify!(#field_names_clone).to_string()),*]
                }
            });

            // implement the get_column_values() function
            column_values_stream.extend(quote! {
                fn get_column_values(&self) -> Vec<String> {
                    vec![#(#field_values),*]
                }
            });

            set_column_values_stream.extend(quote! {
                fn set_column_value(&mut self, column: &str, value: &str) {
                    match column {
                        #(
                            stringify!(#field_names_clone2) => {
                                if let Ok(val) = value.parse::<#field_types_clone>() {
                                    self.#field_names_clone2 = val;
                                } else {
                                    eprintln!("Error: Failed to convert value for column '{}'", column);
                                }
                            }
                        )*

                        // for nested structs, we set their column values
                        $(stringify!($field_types) if $field_types: Table => {
                            <$field_types as Table>::set_column_value(&mut self.$field_names, column, value);
                        })*

                        _ => eprintln!("Warning: Unknown column '{}'", column),
                    }
                }
            });
        }
    };

    let output = quote! {
        impl Table for #ident {
            #name_stream
            #columns_stream
            #column_fields_stream
            #column_values_stream
            #set_column_values_stream
        }
    };

    output.into()
}
