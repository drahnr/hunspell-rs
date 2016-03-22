use std::ffi::CString;

#[test]
fn spell_check() {
    unsafe {
        let affpath = CString::new("/usr/share/hunspell/en_US.aff").unwrap();
        let dicpath = CString::new("/usr/share/hunspell/en_US.dic").unwrap();
        let good_word = CString::new("programming").unwrap();
        let bad_word = CString::new("progrmaing").unwrap();
        let spell = Hunspell_create(affpath.as_ptr(), dicpath.as_ptr());

        assert!(Hunspell_spell(spell, good_word.as_ptr()) == 1);
        assert!(Hunspell_spell(spell, bad_word.as_ptr()) == 0);

        Hunspell_destroy(spell);
    }
}

enum Hunhandle {}

#[link(name = "hunspell")]
extern {
    fn Hunspell_create(affpath: *const i8, dpath: *const i8) -> *mut Hunhandle;
    fn Hunspell_create_key(affpath: *const i8, dpath: *const i8, key: *const i8) -> *mut Hunhandle;
    fn Hunspell_destroy(pHunspell: *mut Hunhandle);

    fn Hunspell_spell(pHunspell: *mut Hunhandle, word: *const i8) -> i32;
}
