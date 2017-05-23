//! write doc here
//!

#![crate_type = "lib"]
#![crate_name = "rlib_depends_on_rust_openssl"]

extern crate bytes;
extern crate crypto as rust_crypto;
extern crate libc;
extern crate openssl;
extern crate rand;

pub mod rlib;
