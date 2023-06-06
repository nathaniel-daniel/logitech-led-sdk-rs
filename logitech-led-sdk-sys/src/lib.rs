#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::unreadable_literal)]

#[cfg(not(any(target_os = "windows", feature = "rustdoc")))]
compile_error!("this library will only work on Windows");

#[cfg(not(any(target_env = "msvc", feature = "rustdoc")))]
compile_error!("this library will only work on the MSVC target environment");

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", feature = "rustdoc")))]
compile_error!("this library will only work on x86 or x86_64");

#[cfg(any(target_arch = "x86_64", feature = "rustdoc"))]
include!("bindings-x86_64.rs");

#[cfg(all(target_arch = "x86", not(feature = "rustdoc")))]
include!("bindings-i686.rs");
