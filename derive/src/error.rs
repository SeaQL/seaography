#[derive(Debug)]
pub enum Error {
    Error(String),
    Syn(syn::Error),
    LexError(proc_macro2::LexError),
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Self::Syn(err)
    }
}

impl From<proc_macro2::LexError> for Error {
    fn from(err: proc_macro2::LexError) -> Self {
        Self::LexError(err)
    }
}
