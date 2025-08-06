use proc_macro2::Span;
use syn::{PathArguments, Token, TypePath};

pub fn qualify_type_path(type_path: &mut TypePath) {
    // convert type path to turbo fish Option<String> -> Option::<String>
    for segment in type_path.path.segments.iter_mut() {
        if let PathArguments::AngleBracketed(arguments) = &mut segment.arguments {
            arguments.colon2_token = Some(Token![::](Span::call_site()));
        }
    }
}
