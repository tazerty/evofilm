//! Matroska (MKV) and WebM demuxer and muxer: a re-export of [`evo_media::matroska`] from
//! evo-libs. Supported elements and limitations are in the evo-media README.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable))]

pub use evo_media::matroska::*;
