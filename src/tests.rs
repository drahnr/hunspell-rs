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

use crate::Hunspell;

#[test]
fn create_and_destroy() {
    let _hs = Hunspell::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic");
}

#[test]
fn check() {
    let hs = Hunspell::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic");
    assert!(hs.check("cats"));
    assert!(!hs.check("nocats"));
}

#[test]
fn check_with_added_word() {
    let mut hs = Hunspell::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic");
    assert!(hs.check("cats"));
    assert!(!hs.check("octonasaurius"));
    assert!(hs.add("octonasaurius"));
    assert!(hs.check("octonasaurius"));
}

#[test]
fn check_with_extra_dic() {
    let mut hs = Hunspell::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic");
    assert!(hs.check("cats"));
    assert!(!hs.check("systemdunits"));
    assert!(hs.add_dictionary("tests/fixtures/extra.dic"));
    assert!(hs.check("cats"));
    assert!(hs.check("systemdunits"));
}

#[test]
fn suggest() {
    let hs = Hunspell::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic");
    assert!(hs.suggest("progra").len() > 0);
}

#[test]
fn stem() {
    let hs = Hunspell::new("tests/fixtures/reduced.aff", "tests/fixtures/reduced.dic");
    let cat_stem = hs.stem("cats");
    assert!(cat_stem[0] == "cat");
}
