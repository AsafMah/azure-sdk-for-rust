#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2021-06-30-preview")]
pub mod package_2021_06_30_preview;
#[cfg(all(feature = "package-2021-06-30-preview", not(feature = "no-default-version")))]
pub use package_2021_06_30_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-12")]
pub mod package_2020_12;
#[cfg(all(feature = "package-2020-12", not(feature = "no-default-version")))]
pub use package_2020_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-version")))]
pub use package_2020_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-03-01-preview")]
pub mod package_2020_03_01_preview;
#[cfg(all(feature = "package-2020-03-01-preview", not(feature = "no-default-version")))]
pub use package_2020_03_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
