#![doc = include_str!("../README.md")]
#![no_std]
#![deny(
    unsafe_code,
    unused_imports,
    unused_variables,
    unused_must_use,
    missing_docs,
    clippy::all,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::dbg_macro,
    clippy::todo,
    clippy::unimplemented
)]

pub(crate) mod utils;

mod candle_stick;
pub use candle_stick::CandleStick;

mod candle_stream;
pub use candle_stream::CandleStream;
