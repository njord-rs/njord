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

extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

use proc_macro2::{Delimiter, TokenTree as TokenTree2};
use quote::quote;

use util::{extract_table_name, has_default_impl};

mod util;

/// Derives the `Table` trait for a struct.
///
/// This procedural macro generates an implementation of the `Table` trait for a struct,
/// enabling interaction with database tables. It derives functionality based on the struct's
/// field names and types, automatically mapping them to corresponding SQL column definitions.
///
/// # Example
///
/// ```rust
/// use njord_derive::Table;
///
/// #[derive(Table)]
/// #[table_name = "users"]
/// struct User {
///     id: i32,
///     name: String,
///     email: Option<String>,
/// }
/// ```
///
/// The `Table` trait will provide:
/// - `get_name()` - Returns the table name.
/// - `get_columns()` - Returns column names and their SQL types.
/// - `get_column_fields()` - Returns the field names as a vector.
/// - `get_column_values()` - Returns the field values as strings.
/// - `set_column_value()` - Updates a field value by column name.
/// - `is_auto_increment_primary_key()` - Checks if a value is an auto-increment primary key.
///
/// Additional traits like `Default`, `Display`, and `FromStr` are also implemented if applicable.
#[proc_macro_derive(Table, attributes(table_name))]
pub fn table_derive(input: TokenStream) -> TokenStream {
    let cloned_input = input.clone();
    let derive_input: DeriveInput = parse_macro_input!(cloned_input);
    let DeriveInput {
        ident, data, attrs, ..
    } = derive_input.clone();

    let table_name = extract_table_name(&attrs);

    let mut name_stream = TokenStream2::default();
    let mut columns_stream = TokenStream2::default();
    let mut column_fields_stream = TokenStream2::default();
    let mut column_values_stream = TokenStream2::default();
    let mut set_column_values_stream = TokenStream2::default();
    let mut is_auto_increment_primary_key_stream = TokenStream2::default();

    let mut display_impl = TokenStream2::default();
    let mut from_str_impl = TokenStream2::default();
    let mut default_impl = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let field_names = named.iter().map(|f| &f.ident);
            let field_names_clone2 = field_names.clone();
            let field_names_clone3 = field_names.clone();
            let field_names_clone4 = field_names.clone();
            let field_types = named.iter().map(|f| &f.ty);
            let field_types_clone = named.iter().map(|f| &f.ty);
            let field_values = named.iter().map(|f| {
                let field_name = &f.ident;
                quote! { self.#field_name.to_string() }
            }); // field_values

            // Implement the std::fmt::Display trait
            display_impl.extend(quote! {
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.get_name())?;
                        for (name, value) in self.get_column_fields().iter().zip(self.get_column_values()) {
                            write!(f, ", {}: {}", name, value)?;
                        }
                        Ok(())
                    }
                }
            }); // display_impl

            // Implement the std::str::FromStr trait
            from_str_impl.extend(quote! {
                impl std::str::FromStr for #ident {
                    type Err = std::string::ParseError;

                    // Creating a new instance of a type from a string.
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        // TODO: to be implemented
                        Ok(#ident::default())
                    }
                }
            }); // from_str_impl

            // Implement the get_name() function
            let clean_table_name = table_name.trim_matches(|c| c == '\\' || c == '"');
            name_stream.extend::<TokenStream2>(quote! {
                fn get_name(&self) -> &str {
                    #clean_table_name
                }
            }); // name_stream

            // Implement the get_columns() function
            columns_stream.extend(quote! {
                fn get_columns(&self) -> std::collections::HashMap<String, String> {
                    let mut columns = std::collections::HashMap::new();
                    #(
                        let column_type = match stringify!(#field_types) {
                            "i64" | "i32" | "i16" | "i8" | "u64" | "u32" | "u16" | "u8" | "usize" => "INTEGER",
                            "String" => "TEXT",
                            "f64" | "f32" => "REAL",
                            "Vec<u8>" => "BLOB",
                            "Option<i64>" | "Option<i32>" | "Option<i16>" | "Option<i8>" | "Option<u64>" | "Option<u32>" | "Option<u16>" | "Option<u8>" | "Option<usize>" => "INTEGER NULL",
                            "Option<String>" => "TEXT NULL",
                            "Option<f64>" | "Option<f32>" => "REAL NULL",
                            "Option<Vec<u8>>" => "BLOB NULL",
                            "bool" => "TEXT",
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
            }); // columns_stream

            // Implement the get_column_fields() function
            column_fields_stream.extend(quote! {
                fn get_column_fields(&self) -> Vec<String> {
                    vec![#(stringify!(#field_names_clone2).to_string()),*]
                }
            }); // column_fields_stream

            // Implement the get_column_values() function
            column_values_stream.extend(quote! {
                fn get_column_values(&self) -> Vec<String> {
                    vec![#(#field_values),*]
                }
            }); // column_values_stream

            set_column_values_stream.extend(quote! {
                fn set_column_value(&mut self, column: &str, value: &str) {
                    match column {
                        #(
                            stringify!(#field_names_clone3) => {
                                if let Ok(val) = value.parse::<#field_types_clone>() {
                                    self.#field_names_clone3 = val;
                                } else {
                                    eprintln!("Error: Failed to convert value for column '{}'", column);
                                }
                            }
                        )*

                        _ => eprintln!("Warning: Unknown column '{}'", column),
                    }
                }
            }); // set_column_values_stream

            // Implement the is_auto_increment_primary_key function
            is_auto_increment_primary_key_stream.extend(quote! {
                fn is_auto_increment_primary_key(&self, value: &str) -> bool {
                    value == "NULL"
                }
            });

            // If Default trait is not implemented, generate an implementation
            default_impl = if !has_default_impl(&derive_input) {
                quote! {
                    impl Default for #ident {
                        fn default() -> Self {
                            Self {
                                #(
                                    #field_names_clone4: Default::default(),
                                )*
                            }
                        }
                    }
                }
            } else {
                TokenStream2::new()
            }; // default_impl
        }
    };

    let output = quote! {
        impl Table for #ident {
            #name_stream
            #columns_stream
            #column_fields_stream
            #column_values_stream
            #set_column_values_stream
            #is_auto_increment_primary_key_stream
        }

        #default_impl
        #display_impl
        #from_str_impl
    };

    output.into()
}

/// A procedural macro `sql!` that takes SQL-like syntax and transforms it into a formatted string.
///
/// # Example
///
/// ```rust
/// use njord_derive::sql;
/// let id = 1;
///
/// let query = sql! {
///     SELECT * FROM user WHERE id = {id}
/// };
/// assert_eq!(query, "SELECT * FROM user WHERE id = 1");
/// ```
///
/// This macro supports embedding expressions within SQL queries, ensuring proper formatting
/// for identifiers and values.
///
/// Note:
/// - Use `{}` syntax for expressions to embed Rust variables.
/// - Identifiers are automatically quoted if necessary.
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let mut tokens = input.into_iter().peekable();
    let mut sql_parts = Vec::new();
    let mut expressions = Vec::new();
    let mut param_types = Vec::new();
    let mut current_sql = String::new();
    let mut last_token_type = TokenType::Other;

    #[derive(PartialEq, Clone)]
    enum TokenType {
        Dot,
        OpenParen,
        CloseParen,
        Operator,
        Other,
    }

    while let Some(token) = tokens.next() {
        match token {
            TokenTree2::Group(group) if group.delimiter() == Delimiter::Brace => {
                if !current_sql.is_empty() {
                    sql_parts.push(current_sql);
                    current_sql = String::new();
                }

                // Parse the expression to determine its type
                let expr = group.stream();
                let expr_str = expr.to_string();

                // Check if it's an identifier (likely a string variable)
                let needs_quotes = !expr_str.contains("as")
                    && !expr_str.contains("::")
                    && !expr_str.starts_with("Some")
                    && !expr_str.parse::<f64>().is_ok()
                    && !expr_str.parse::<i64>().is_ok();

                if needs_quotes {
                    sql_parts.push("'{}'".to_string());
                } else {
                    sql_parts.push("{}".to_string());
                }

                expressions.push(expr);
                param_types.push(needs_quotes);
                last_token_type = TokenType::Other;
            }
            token => {
                let token_str = token.to_string();
                let current_token_type = match token_str.as_str() {
                    "." => TokenType::Dot,
                    "(" => TokenType::OpenParen,
                    ")" => TokenType::CloseParen,
                    "=" | ">" | "<" | ">=" | "<=" | "!=" => TokenType::Operator,
                    _ => TokenType::Other,
                };
                match current_token_type {
                    TokenType::Dot => {
                        current_sql.push('.');
                    }
                    TokenType::OpenParen => {
                        current_sql.push('(');
                    }
                    TokenType::CloseParen => {
                        current_sql.push(')');
                        if let Some(next) = tokens.peek() {
                            let next_str = next.to_string();
                            if !matches!(next_str.as_str(), "," | "." | ")" | ";") {
                                current_sql.push(' ');
                            }
                        }
                    }
                    TokenType::Operator => {
                        if !current_sql.ends_with(' ') {
                            current_sql.push(' ');
                        }
                        current_sql.push_str(&token_str);
                        current_sql.push(' ');
                    }
                    TokenType::Other => {
                        let needs_space = !current_sql.is_empty()
                            && !current_sql.ends_with(' ')
                            && !matches!(last_token_type, TokenType::Dot | TokenType::OpenParen)
                            && token_str != ","
                            && token_str != ";";
                        if needs_space {
                            current_sql.push(' ');
                        }
                        current_sql.push_str(&token_str);
                        if token_str == "," {
                            current_sql.push(' ');
                        }
                    }
                }
                last_token_type = current_token_type;
            }
        }
    }

    if !current_sql.is_empty() {
        sql_parts.push(current_sql);
    }

    let sql_format = sql_parts.join("");
    let expanded = if expressions.is_empty() {
        quote! {
            #sql_format.to_string()
        }
    } else {
        quote! {
            format!(#sql_format #(,#expressions)*)
        }
    };

    expanded.into()
}
