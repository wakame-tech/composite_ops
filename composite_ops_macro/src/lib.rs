extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(CompositeOps)]
pub fn derive_composite_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    dbg!(&item);
    let struct_name = item.ident;
    let field_name = item.fields.iter().map(|f| f.ident.clone());
    let gen = quote! {
        impl std::ops::Add for #struct_name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                #struct_name {
                    #(
                        #field_name: self.#field_name + rhs.#field_name,
                    )*
                }
            }
        }
    };
    gen.into()
}
