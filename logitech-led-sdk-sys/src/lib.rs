#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::unreadable_literal)]

#[cfg(not(target_os = "windows"))]
compile_error!("this library will only work on Windows");

#[cfg(not(target_env = "msvc"))]
compile_error!("this library will only work on the MSVC target environment");

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
compile_error!("this library will only work on x86 or x86_64");

#[cfg(target_arch = "x86_64")]
include!("bindings-x86_64.rs");

#[cfg(target_arch = "x86")]
include!("bindings-i686.rs");
