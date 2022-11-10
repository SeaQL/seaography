use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn enum_filter_fn(ident: syn::Ident) -> TokenStream {
    let name = format_ident!("{}EnumFilter", ident);

    quote! {
        #[derive(Debug, Clone, async_graphql::InputObject)]
        pub struct #name {
            pub eq: Option<#ident>,
            pub ne: Option<#ident>,
            pub gt: Option<#ident>,
            pub gte: Option<#ident>,
            pub lt: Option<#ident>,
            pub lte: Option<#ident>,
            pub is_in: Option<Vec<#ident>>,
            pub is_not_in: Option<Vec<#ident>>,
            pub is_null: Option<bool>,
        }

        impl seaography::FilterTrait for #name {
            type Ty = #ident;

            fn eq(&self) -> Option<Self::Ty> {
                self.eq.clone()
            }
            fn ne(&self) -> Option<Self::Ty> {
                self.ne.clone()
            }
            fn gt(&self) -> Option<Self::Ty> {
                self.gt.clone()
            }
            fn gte(&self) -> Option<Self::Ty> {
                self.gte.clone()
            }
            fn lt(&self) -> Option<Self::Ty> {
                self.lt.clone()
            }
            fn lte(&self) -> Option<Self::Ty> {
                self.lte.clone()
            }
            fn is_in(&self) -> Option<Vec<Self::Ty>> {
                self.is_in.clone()
            }
            fn is_not_in(&self) -> Option<Vec<Self::Ty>> {
                self.is_not_in.clone()
            }
            fn is_null(&self) -> Option<bool> {
                self.is_null
            }
            fn contains(&self) -> Option<String> {
                panic!("contains not supported for enumerations")
            }
            fn starts_with(&self) -> Option<String> {
                panic!("starts_with not supported for enumerations")
            }
            fn ends_with(&self) -> Option<String> {
                panic!("ends_with not supported for enumerations")
            }
            fn like(&self) -> Option<String> {
                panic!("like not supported for enumerations")
            }
            fn not_like(&self) -> Option<String> {
                panic!("not_like not supported for enumerations")
            }
        }

        impl seaography::FilterTypeTrait for #ident {
            type Filter = #name;
        }
    }
}
