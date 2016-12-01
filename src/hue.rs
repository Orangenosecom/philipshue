#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/hue.rs"));

#[cfg(feature = "serde_derive")]
include!("hue.in.rs");
