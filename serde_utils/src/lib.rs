#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub mod prelude {
    pub use alloc::format;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::vec;
    pub use alloc::vec::Vec;
    pub use core::fmt;
    pub use core::str::FromStr;
}

#[cfg(feature = "std")]
pub mod prelude {
    pub use core::fmt;
    pub use core::str::FromStr;
}

mod quoted_int;

pub mod fixed_bytes_hex;
pub mod hex;
pub mod hex_vec;
pub mod json_str;
pub mod list_of_bytes_lists;
pub mod quoted_u64_vec;
pub mod u256_hex_be;
pub mod u32_hex;
pub mod u64_hex_be;
pub mod u8_hex;

pub use fixed_bytes_hex::{bytes_4_hex, bytes_8_hex};
pub use quoted_int::{quoted_i64, quoted_u256, quoted_u32, quoted_u64, quoted_u8};
