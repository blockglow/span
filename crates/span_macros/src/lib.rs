use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, LitStr};

#[proc_macro_attribute]
pub fn data(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let copy = proc_macro2::TokenStream::from(_item.clone());
    let id = parse_macro_input!(_attr as LitStr);
    let item = parse_macro_input!(_item as ItemEnum);
    let item_ident = item.ident;
    let result = quote! {
        #copy

        impl span::Datatype for #item_ident {
            fn id() -> uuid::Uuid {
                Uuid::from_str(#id).unwrap()
            }
        }
    };
    result.into()
}
