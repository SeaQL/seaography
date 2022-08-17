/// Used to extract enumerations from table meta
///
/// ```
/// use quote::quote;
/// use seaography_generator::files::entity::extract_enums;
/// use seaography_generator::test_cfg::get_font_table;
///
/// let left: Vec<_> = extract_enums(&get_font_table()).iter().map(|t| t.to_string()).collect();
///
/// let right: Vec<_> = vec![
///     quote! {
///         LanguageEnum
///     },
///     quote! {
///         VariantEnum
///     },
///
/// ].iter().map(|t| t.to_string()).collect();
///
/// assert_eq!(left, right);
/// ```
pub fn extract_enums(table_meta: &TableMeta) -> Vec<TokenStream> {
    table_meta
        .columns
        .iter()
        .filter(|col| matches!(col.col_type, ColumnType::Enum(_)))
        .map(|col| {
            if let ColumnType::Enum(name) = &col.col_type {
                name.to_upper_camel_case().parse().unwrap()
            } else {
                panic!("UNREACHABLE")
            }
        })
        .collect()
}