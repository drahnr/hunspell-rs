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
