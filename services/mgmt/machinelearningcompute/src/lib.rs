#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2017-08-preview")]
pub mod package_2017_08_preview;
#[cfg(all(feature = "package-2017-08-preview", not(feature = "no-default-version")))]
pub use package_2017_08_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-06-preview")]
pub mod package_2017_06_preview;
#[cfg(all(feature = "package-2017-06-preview", not(feature = "no-default-version")))]
pub use package_2017_06_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
