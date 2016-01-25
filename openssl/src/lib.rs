#![doc(html_root_url="https://sfackler.github.io/rust-openssl/doc/v0.7.5")]
#![cfg_attr(feature = "nightly", feature(const_fn, recover, panic_propagate))]

#[macro_use]
extern crate bitflags;
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate openssl_sys as ffi;
extern crate openssl_sys_extras as ffi_extras;
extern crate time;

#[cfg(test)]
extern crate rustc_serialize as serialize;

#[cfg(test)]
extern crate net2;

mod macros;

pub mod asn1;
pub mod bn;
pub mod bio;
pub mod crypto;
pub mod dh;
pub mod ssl;
pub mod x509;
pub mod nid;
