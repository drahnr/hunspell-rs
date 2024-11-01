# Spell checking, stemming, morphological generation and analysis with Hunspell.

This crate provides a Rust interface to the [Hunspell library]
using the [hunspell-sys] crate.

The Hunspell library routines give the user word-level linguistic
functions: spell checking and correction, stemming, morphological
generation and analysis.

## Example

```rust
use hunspell_rs::SpellChecker;

let spell = SpellChecker::new("en_UK.aff", "en_UK.dic").unwrap();
assert_eq!(Ok(true), spell.check("cats"));
assert_eq!(Ok(false),spell.check("nocats"));
```

## Features

- **bundeled** The bundled code of hunspell can be compiled with the `cc`
  crate and will be linked `static`ally when the `bundled` feature is
  present (default).
- **serde** Serialize/deserialize the hunspell `SpellChecker`.

## To do

-[ ] Unimplemented from hunspell-sys: Hunspell_get_dic_encoding
-[ ] Improve documentation
-[ ] Make SpellCheck lazy (only load dictionaries on use).
-[ ] More tests
-[ ] Cache added words so they can persist using serde.

[Hunspell library]: https://hunspell.github.io/
[hunspell-sys]: https://crates.io/crates/hunspell-sys
