#![warn(
    clippy::all,
    //clippy::restriction,
    clippy::pedantic,
    //clippy::nursery,
    //clippy::cargo
)]
extern crate core;

pub mod ck {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(clippy::pub_underscore_fields)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod error_types;
pub mod welcome_protocol;
pub mod environment_protocol;
pub mod constants;
pub mod util;
pub mod message_parser;
pub mod shareable_spsc_ring_buffer;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
    }
}