//   Copyright 2016 Lipka Boldizsár
//   Copyright 2019 Alberto Piai
//   Copyright 2020 Bernhard Schuster
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use crate::SpellChecker;

#[test]
fn create_and_destroy() {
    let _hs =
        SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
}

#[test]
fn check() {
    let hs = SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
    assert_eq!(Ok(true), hs.check("cats"));
    assert_eq!(Ok(false), hs.check("nocats"));
}

#[test]
fn spell_with_add_and_remove() {
    let hs = SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
    assert_eq!(Ok(true), hs.check("cats"));
    assert_eq!(Ok(false), hs.check("octonasaurius"));
    assert_eq!(Ok(()), hs.add("octonasaurius"));
    assert_eq!(Ok(true), hs.check("octonasaurius"));
    assert_eq!(Ok(()), hs.remove("octonasaurius"));
    assert_eq!(Ok(false), hs.check("octonasaurius"));
}

#[test]
fn spell_with_add_with_affix() {
    let hs = SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
    assert_eq!(Ok(true), hs.check("cats"));
    assert_eq!(Ok(false), hs.check("rusts"));
    assert_eq!(Ok(()), hs.add_with_affix("rust", "cat"));
    assert_eq!(Ok(true), hs.check("rusts"));
}

#[test]
fn spell_with_extra_dic() {
    let mut hs =
        SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
    assert_eq!(Ok(true), hs.check("cats"));
    assert_eq!(Ok(false), hs.check("systemdunits"));
    assert_eq!(Ok(true), hs.add_dictionary("tests/fixtures/extra.dic"));
    assert_eq!(Ok(true), hs.check("cats"));
    assert_eq!(Ok(true), hs.check("systemdunits"));
}

#[test]
fn suggest() {
    let hs = SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
    assert!(hs.suggest("progra").unwrap().len() > 0);
}

#[test]
fn stem() {
    let hs = SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
    let cat_stem = hs.stem("cats").unwrap();
    assert!(cat_stem[0] == "cat");
}

#[test]
#[cfg(feature = "serde")]
fn serde() {
    let hs = SpellChecker::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic").unwrap();
    let serialized: Vec<u8> = bincode::serialize(&hs).unwrap();
    let deserialized: SpellChecker = bincode::deserialize(&serialized[..]).unwrap();
    assert_eq!(hs.affix(), deserialized.affix());
    let cat_stem = deserialized.stem("cats").unwrap();
    assert!(cat_stem[0] == "cat");
}
