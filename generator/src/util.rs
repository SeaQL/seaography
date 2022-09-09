use quote::ToTokens;

pub(crate) fn add_line_break(content: proc_macro2::TokenStream) -> String {
    let file_parsed: syn::File = syn::parse2(content).unwrap();
    let blocks: Vec<String> =
        file_parsed
            .items
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, item)| {
                let mut s = item.into_token_stream().to_string();
                if !acc.is_empty() && no_line_break_in_between(&file_parsed.items[i - 1], item) {
                    let last = acc.swap_remove(acc.len() - 1);
                    s = format!("{}{}", last, s);
                }
                acc.push(s);
                acc
            });
    replace_fully_qualified_spaces(blocks.join("\n\n"))
}

pub(crate) fn no_line_break_in_between(this: &syn::Item, that: &syn::Item) -> bool {
    match (this, that) {
        (syn::Item::Mod(_), syn::Item::Mod(_)) | (syn::Item::Use(_), syn::Item::Use(_)) => true,
        _ => false,
    }
}

pub(crate) fn replace_fully_qualified_spaces(mut str: String) -> String {
    let targets = [
        ("seaography :: macros :: ", "seaography::macros::"),
        ("async_graphql :: ", "async_graphql::"),
    ];
    for (from, to) in targets {
        str = str.replace(from, to);
    }
    str
}
