#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/json.rs"));

#[cfg(feature = "serde_derive")]
include!("json.in.rs");
