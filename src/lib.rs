//! Spell checking, stemming, morphological generation and analysis with hunspell.
//!
//! This crate provides a Rust interface to the [Hunspell library]
//! using the [hunspell-sys] crate.
//!
//! The Hunspell library routines give the user word-level linguistic
//! functions: spell checking and correction, stemming, morphological
//! generation and analysis.
//!
//! # Example
//!
//! ```
//! use hunspell_rs::SpellChecker;
//!
//! let spell = SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
//! assert_eq!(Ok(true), spell.check("cats"));
//! assert_eq!(Ok(false),spell.check("nocats"));
//! ```
//!
//! # Getting dictionaries
//!
//! - Libre office: git://anongit.freedesktop.org/libreoffice/dictionaries
//! - [Collection of normalized and installable hunspell dictionaries](https://github.com/wooorm/dictionaries)
//!
//! # Features
//!
//! - **bundeled** The bundled code of hunspell can be compiled with the `cc`
//!   crate and will be linked `static`ally when the `bundled` feature is
//!   present (default).
//! - **serde** Serialize/deserialize the hunspell [`Dictionary`].
//!
//! [Hunspell library]: https://hunspell.github.io/
//! [hunspell-sys]: https://crates.io/crates/hunspell-sys
mod error;
mod spell_checker;

#[cfg(feature = "serde")]
mod serde;

pub use error::{Error, Result};
pub use spell_checker::SpellChecker;

#[cfg(test)]
mod tests;
