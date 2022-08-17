use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;

/// Used to generate code for orm_dataloader.rs
///
/// ```
/// use quote::quote;
/// use seaography_generator::extended::orm_dataloader::generate_orm_dataloader;
///
/// let left = generate_orm_dataloader();
///
/// let right = quote! {
///     use sea_orm::prelude::*;
///
///     pub struct OrmDataloader {
///         pub db: DatabaseConnection,
///     }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_orm_dataloader() -> TokenStream {
    quote! {
        use sea_orm::prelude::*;

        pub struct OrmDataloader {
            pub db: DatabaseConnection,
        }
    }
}

pub fn write_orm_dataloader<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let file_name = path.as_ref().join("orm_dataloader.rs");

    let data = generate_orm_dataloader();

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
