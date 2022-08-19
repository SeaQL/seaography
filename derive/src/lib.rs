use syn::DeriveInput;

mod filter;

#[proc_macro_derive(Filter, attributes(sea_attr))]
pub fn derive_filter_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let item = match data {
        syn::Data::Struct(item) => item,
        _ => panic!("Input not structure")
    };

    if ident.ne("Model") {
        panic!("Struct must be ORM model")
    }

    let attrs = filter::SeaAttr::from_attributes(&attrs).unwrap();

    filter::filter_fn(item, attrs).into()
}



#[proc_macro_derive(Relations)]
pub fn derive_relations_fn(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // println!("Relations: {:?}", item);
    "fn answer1() -> u32 { 42 }".parse().unwrap()
}