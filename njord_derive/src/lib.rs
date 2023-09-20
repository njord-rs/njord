extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Table)]
pub fn table_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        impl Table for #name {
            fn get_name(&self) -> &str {
                return &self.name;
            }

            fn get_columns(&self) -> &HashMap<String, String> {
                return &self.columns;
            }

            fn get_column_fields(&self) -> Vec<String> {
                let mut columns = Vec::new();
                for (column_name, _column_type) in self.get_columns() {
                    columns.push(column_name.to_string());
                }

                columns
            }
        }
    };

    gen.into()
}
