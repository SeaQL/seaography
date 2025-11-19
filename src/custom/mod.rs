#[cfg(not(feature = "strict-custom-types"))]
mod aux_types;
#[cfg(feature = "strict-custom-types")]
mod impls;
mod input;
mod output;
mod types;

#[cfg(not(feature = "strict-custom-types"))]
pub use aux_types::*;
pub use input::*;
pub use output::*;
pub use types::*;
