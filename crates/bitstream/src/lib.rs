//! Bit-level reading and writing: a re-export of [`evo_media::bitstream`] from evo-libs.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable))]

pub use evo_media::bitstream::*;
