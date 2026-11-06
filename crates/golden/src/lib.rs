//! Golden-image tests for EvoFilm's renderer (test-only crate; the library is empty).
//!
//! `tests/golden.rs` builds small procedural projects (no media files), renders one frame of each
//! through the CPU reference compositor (`evofilm-render`) and compares it with a committed PNG
//! in `goldens/` using `evofilm_testkit::golden`. The same scenes are composited on the GPU
//! (`evofilm-gpu`) when an adapter is available and compared with the CPU result.
//!
//! Regenerate references with `EVOFILM_BLESS=1 cargo test -p evofilm-golden`.
