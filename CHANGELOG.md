# Changes for Hunspell

## 0.4.0 -> 0.5.0

- BREAKING: added error handling, most methods now return a Result instead
  of panicking
- BREAKING: changed main struct name from Hunspell to SpellChecker
- Additional methods to hunspell-sys implemented.
- No longer uses macro to generate code to the pointer to Vec conversions. 
- Added documentation.
- Manual Clone to prevent dangling pointer
- Optional serde Serialize/Deserialize
